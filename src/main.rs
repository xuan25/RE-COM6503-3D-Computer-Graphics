//! Rust/GLFW port of the legacy JOGL Museum.  It intentionally loads `legacy` assets in place.
#![allow(unsafe_op_in_unsafe_fn)]
use glfw::{Action, Context, Key, WindowEvent, fail_on_errors};
use std::{
    ffi::{CString, c_void},
    path::Path,
    ptr,
    time::Instant,
};
const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/legacy");
type M = [f32; 16];
type V = [f32; 3];
fn id() -> M {
    [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ]
}
fn mm(a: M, b: M) -> M {
    let mut r = [0.; 16];
    for c in 0..4 {
        for y in 0..4 {
            r[c * 4 + y] = (0..4).map(|x| a[x * 4 + y] * b[c * 4 + x]).sum()
        }
    }
    r
}
fn t(v: V) -> M {
    let mut m = id();
    m[12] = v[0];
    m[13] = v[1];
    m[14] = v[2];
    m
}
fn s(v: V) -> M {
    [
        v[0], 0., 0., 0., 0., v[1], 0., 0., 0., 0., v[2], 0., 0., 0., 0., 1.,
    ]
}
fn rx(a: f32) -> M {
    let (s, c) = a.to_radians().sin_cos();
    [1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.]
}
fn ry(a: f32) -> M {
    let (s, c) = a.to_radians().sin_cos();
    [c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.]
}
fn rz(a: f32) -> M {
    let (s, c) = a.to_radians().sin_cos();
    [c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.]
}
fn xform(p: V, r: V, z: V) -> M {
    mm(t(p), mm(ry(r[1]), mm(rx(r[0]), mm(rz(r[2]), s(z)))))
}
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: V, b: V) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn unit(v: V) -> V {
    let q = dot(v, v).sqrt();
    [v[0] / q, v[1] / q, v[2] / q]
}
fn proj(aspect: f32) -> M {
    let f = 1. / (22.5_f32.to_radians()).tan();
    [
        f / aspect,
        0.,
        0.,
        0.,
        0.,
        f,
        0.,
        0.,
        0.,
        0.,
        -1.00067,
        -1.,
        0.,
        0.,
        -0.20007,
        0.,
    ]
}
fn look(e: V, c: V, u: V) -> M {
    let f = unit(sub(c, e));
    let q = unit(cross(f, u));
    let v = cross(q, f);
    [
        q[0],
        v[0],
        -f[0],
        0.,
        q[1],
        v[1],
        -f[1],
        0.,
        q[2],
        v[2],
        -f[2],
        0.,
        -dot(q, e),
        -dot(v, e),
        dot(f, e),
        1.,
    ]
}
struct Camera {
    p: V,
    f: V,
    u: V,
    r: V,
    yaw: f32,
    pitch: f32,
}
impl Camera {
    fn new() -> Self {
        let mut c = Self {
            p: [40., 32., 50.],
            f: [0.; 3],
            u: [0., 1., 0.],
            r: [0.; 3],
            yaw: 0.,
            pitch: 0.,
        };
        c.aim([-3., 2., 0.]);
        c
    }
    fn aim(&mut self, v: V) {
        self.f = unit(sub(v, self.p));
        self.yaw = self.f[2].atan2(self.f[0]);
        self.pitch = self.f[1].asin();
        self.upd()
    }
    fn upd(&mut self) {
        self.f = unit([
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ]);
        self.r = unit(cross(self.f, [0., 1., 0.]));
        self.u = unit(cross(self.r, self.f))
    }
    fn mv(&mut self, d: V, dt: f32) {
        self.p = add(self.p, [d[0] * 12. * dt, d[1] * 12. * dt, d[2] * 12. * dt])
    }
    fn preset(&mut self, n: u8) {
        self.p = match n {
            1 => [0., 0., 25.],
            2 => [0., 25., 0.001],
            _ => [25., 0., 0.],
        };
        self.aim([0., 0., 0.])
    }
}
struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    n: i32,
}
impl Mesh {
    unsafe fn new(v: &[f32], i: &[u32]) -> Self {
        let (mut a, mut b, mut e) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut a);
        gl::GenBuffers(1, &mut b);
        gl::GenBuffers(1, &mut e);
        gl::BindVertexArray(a);
        gl::BindBuffer(gl::ARRAY_BUFFER, b);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(v) as isize,
            v.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, e);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(i) as isize,
            i.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        for (k, n, o) in [(0, 3, 0), (1, 3, 3), (2, 2, 6)] {
            gl::VertexAttribPointer(k, n, gl::FLOAT, gl::FALSE, 8 * 4, (o * 4) as *const c_void);
            gl::EnableVertexAttribArray(k)
        }
        gl::BindVertexArray(0);
        Self {
            vao: a,
            vbo: b,
            ebo: e,
            n: i.len() as i32,
        }
    }
    unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.n, gl::UNSIGNED_INT, ptr::null())
    }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo)
        }
    }
}
struct Tex(u32);
impl Tex {
    unsafe fn load(name: &str) -> Self {
        let p = Path::new(ROOT).join(name);
        let image = image::open(&p)
            .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
            .flipv()
            .to_rgba8();
        let (w, h) = image.dimensions();
        let mut x = 0;
        gl::GenTextures(1, &mut x);
        gl::BindTexture(gl::TEXTURE_2D, x);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::SRGB_ALPHA as i32,
            w as i32,
            h as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        Self(x)
    }
}
impl Drop for Tex {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.0) }
    }
}
unsafe fn sh(k: u32, s: &str) -> u32 {
    let q = gl::CreateShader(k);
    let c = CString::new(s).unwrap();
    gl::ShaderSource(q, 1, &c.as_ptr(), ptr::null());
    gl::CompileShader(q);
    let mut ok = 0;
    gl::GetShaderiv(q, gl::COMPILE_STATUS, &mut ok);
    if ok == 0 {
        let mut b = [0i8; 1024];
        gl::GetShaderInfoLog(q, 1024, ptr::null_mut(), b.as_mut_ptr());
        panic!(
            "GLSL: {}",
            String::from_utf8_lossy(std::slice::from_raw_parts(b.as_ptr().cast(), 1024))
        )
    }
    q
}
unsafe fn pg(v: &str, f: &str) -> u32 {
    let (a, b) = (sh(gl::VERTEX_SHADER, v), sh(gl::FRAGMENT_SHADER, f));
    let p = gl::CreateProgram();
    gl::AttachShader(p, a);
    gl::AttachShader(p, b);
    gl::LinkProgram(p);
    gl::DeleteShader(a);
    gl::DeleteShader(b);
    p
}
unsafe fn l(p: u32, n: &str) -> i32 {
    gl::GetUniformLocation(p, CString::new(n).unwrap().as_ptr())
}
unsafe fn um(p: u32, n: &str, m: &M) {
    gl::UniformMatrix4fv(l(p, n), 1, gl::FALSE, m.as_ptr())
}
unsafe fn uv(p: u32, n: &str, v: V) {
    gl::Uniform3f(l(p, n), v[0], v[1], v[2])
}
const VERT: &str = r#"#version 330 core
layout(location=0)in vec3 a;layout(location=1)in vec3 b;layout(location=2)in vec2 c;out vec3 p;out vec3 n;out vec2 q;uniform mat4 model,vp;uniform vec2 offset,repeat;void main(){p=vec3(model*vec4(a,1));n=mat3(transpose(inverse(model)))*b;q=c*repeat+offset;gl_Position=vp*vec4(p,1);}"#;
const FRAG: &str = r#"#version 330 core
in vec3 p,n;in vec2 q;out vec4 o;uniform sampler2D tex,spec;uniform vec3 cam,day,point[4],spot,spotdir,tint;uniform float spotpower;void main(){vec3 N=normalize(n),V=normalize(cam-p),base=texture(tex,q).rgb*tint,sp=texture(spec,q).rgb;vec3 r=base*(0.05+day*0.45);for(int i=0;i<4;i++){vec3 d=point[i]-p;float z=length(d);vec3 L=d/z;float hi=pow(max(dot(N,normalize(L+V)),0.),16.);r+=(base*max(dot(N,L),0.)+sp*hi)*vec3(1.,0.9,0.7)/(1.+0.018*z*z);}vec3 d=spot-p;float z=length(d);vec3 L=d/z;float cone=smoothstep(0.75,0.9,dot(normalize(-spotdir),L));r+=(base*max(dot(N,L),0.)+sp*0.2)*cone*spotpower/(1.+0.025*z*z);o=vec4(r,1);}"#;
const SKYV: &str = r#"#version 330 core
layout(location=0)in vec3 a;layout(location=2)in vec2 c;out vec2 q;uniform mat4 vp;uniform vec2 offset;void main(){gl_Position=(vp*vec4(a,1)).xyww;q=c+offset;}"#;
const SKYF: &str = r#"#version 330 core
in vec2 q;out vec4 o;uniform sampler2D tex;uniform vec3 tint;void main(){o=vec4(texture(tex,q).rgb*tint,1);}"#;
const POSTV: &str = r#"#version 330 core
layout(location=0)in vec3 a;layout(location=1)in vec3 b;out vec2 q;void main(){gl_Position=vec4(a.xy,0,1);q=b.yz;}"#;
const POSTF: &str = r#"#version 330 core
in vec2 q;out vec4 o;uniform sampler2D tex;void main(){vec3 a=texture(tex,q).rgb;a=vec3(1)-exp(-a*2.5);o=vec4(pow(a,vec3(1./2.2)),1);}"#;
fn cube() -> (Vec<f32>, Vec<u32>) {
    let p = [
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
    ];
    let f = [
        ([0, 1, 3, 2], [-1., 0., 0.]),
        ([4, 6, 7, 5], [1., 0., 0.]),
        ([1, 5, 7, 3], [0., 0., 1.]),
        ([0, 2, 6, 4], [0., 0., -1.]),
        ([0, 4, 5, 1], [0., -1., 0.]),
        ([3, 7, 6, 2], [0., 1., 0.]),
    ];
    let (mut v, mut i) = (Vec::new(), Vec::new());
    for (k, n) in f {
        let b = (v.len() / 8) as u32;
        for (j, u) in k.into_iter().zip([[0., 0.], [1., 0.], [1., 1.], [0., 1.]]) {
            v.extend_from_slice(&[p[j][0], p[j][1], p[j][2], n[0], n[1], n[2], u[0], u[1]])
        }
        i.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3])
    }
    (v, i)
}
fn plane() -> (Vec<f32>, Vec<u32>) {
    (
        vec![
            -0.5, 0., -0.5, 0., 1., 0., 0., 1., -0.5, 0., 0.5, 0., 1., 0., 0., 0., 0.5, 0., 0.5,
            0., 1., 0., 1., 0., 0.5, 0., -0.5, 0., 1., 0., 1., 1.,
        ],
        vec![0, 1, 2, 0, 2, 3],
    )
}
fn sphere() -> (Vec<f32>, Vec<u32>) {
    let (mut v, mut i) = (Vec::new(), Vec::new());
    for y in 0..30 {
        let b = (-90. + 180. * y as f32 / 29.).to_radians();
        for x in 0..30 {
            let a = (360. * x as f32 / 29.).to_radians();
            let p = [b.cos() * a.sin(), b.sin(), b.cos() * a.cos()];
            v.extend_from_slice(&[
                p[0] * 0.5,
                p[1] * 0.5,
                p[2] * 0.5,
                p[0],
                p[1],
                p[2],
                x as f32 / 29.,
                y as f32 / 29.,
            ])
        }
    }
    for y in 0..29 {
        for x in 0..29 {
            let a = (y * 30 + x) as u32;
            i.extend_from_slice(&[a, a + 1, a + 30, a, a + 30, a + 31])
        }
    }
    (v, i)
}
struct A {
    cube: Mesh,
    plane: Mesh,
    sphere: Mesh,
    floor: Tex,
    wall: Tex,
    wood: Tex,
    paint: Tex,
    robot: Tex,
    metal: Tex,
    marble: Tex,
    phone: Tex,
    window: Tex,
    snow: Tex,
    sky: Tex,
    white: Tex,
}
unsafe fn make_a() -> A {
    let (c, ci) = cube();
    let (p, pi) = plane();
    let (s, si) = sphere();
    A {
        cube: Mesh::new(&c, &ci),
        plane: Mesh::new(&p, &pi),
        sphere: Mesh::new(&s, &si),
        floor: Tex::load("textures/Wood_Plank_vgwnadk_2K_Albedo.jpg"),
        wall: Tex::load("textures/Wood_Other_ugclefmn_2K_Albedo.jpg"),
        wood: Tex::load("textures/Wood_Board_vigjfivg_2K_Albedo.jpg"),
        paint: Tex::load("textures/Paintings_Abstract_qirpc_2K_Albedo.jpg"),
        robot: Tex::load("textures/Metal_Painted_vbsieik_2K_Albedo.jpg"),
        metal: Tex::load("textures/Metal_td1kaean_2K_Albedo.jpg"),
        marble: Tex::load("textures/Marble_Polished_ufojbjkl_2K_Albedo.jpg"),
        phone: Tex::load("textures/homtom-ht7-released-02.jpg"),
        window: Tex::load("textures/iHlkbr8-mt-fuji-wallpaper.jpg"),
        snow: Tex::load("textures/snow.jpg"),
        sky: Tex::load("textures/SkyhighFluffycloudField4k.jpg"),
        white: Tex::load("textures/white.jpg"),
    }
}
unsafe fn draw(
    p: u32,
    m: &Mesh,
    model: M,
    vp: M,
    c: &Camera,
    a: &Tex,
    b: &Tex,
    day: V,
    spot: V,
    dir: V,
    wire: bool,
    off: [f32; 2],
    rep: [f32; 2],
) {
    gl::UseProgram(p);
    um(p, "model", &model);
    um(p, "vp", &vp);
    uv(p, "cam", c.p);
    uv(p, "day", day);
    uv(p, "spot", spot);
    uv(p, "spotdir", dir);
    uv(p, "tint", [1., 1., 1.]);
    for (i, v) in [
        [-6., 12., -6.],
        [-6., 12., 6.],
        [6., 12., -6.],
        [6., 12., 6.],
    ]
    .into_iter()
    .enumerate()
    {
        uv(p, &format!("point[{i}]"), v)
    }
    gl::Uniform1f(l(p, "spotpower"), 1.);
    gl::Uniform2f(l(p, "offset"), off[0], off[1]);
    gl::Uniform2f(l(p, "repeat"), rep[0], rep[1]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, a.0);
    gl::Uniform1i(l(p, "tex"), 0);
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, b.0);
    gl::Uniform1i(l(p, "spec"), 1);
    gl::PolygonMode(gl::FRONT_AND_BACK, if wire { gl::LINE } else { gl::FILL });
    m.draw()
}
struct Fbo {
    msaa: u32,
    mc: u32,
    md: u32,
    hdr: u32,
    hc: u32,
    hd: u32,
    quad: Mesh,
    w: i32,
    h: i32,
}
impl Fbo {
    unsafe fn new() -> Self {
        let v = [
            -1., 1., 0., 0., 1., 0., 0., 0., -1., -1., 0., 0., 0., 0., 0., 0., 1., -1., 0., 1., 0.,
            0., 0., 0., -1., 1., 0., 0., 1., 0., 0., 0., 1., -1., 0., 1., 0., 0., 0., 0., 1., 1.,
            0., 1., 1., 0., 0., 0.,
        ];
        let ix = [0, 1, 2, 3, 4, 5];
        let q = Mesh::new(&v, &ix);
        Self {
            msaa: 0,
            mc: 0,
            md: 0,
            hdr: 0,
            hc: 0,
            hd: 0,
            quad: q,
            w: 0,
            h: 0,
        }
    }
    unsafe fn resize(&mut self, w: i32, h: i32) {
        if (w, h) == (self.w, self.h) {
            return;
        }
        if self.msaa > 0 {
            gl::DeleteFramebuffers(1, &self.msaa);
            gl::DeleteTextures(1, &self.mc);
            gl::DeleteRenderbuffers(1, &self.md);
            gl::DeleteFramebuffers(1, &self.hdr);
            gl::DeleteTextures(1, &self.hc);
            gl::DeleteRenderbuffers(1, &self.hd)
        }
        self.w = w;
        self.h = h;
        gl::GenFramebuffers(1, &mut self.msaa);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.msaa);
        gl::GenTextures(1, &mut self.mc);
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.mc);
        gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, 16, gl::RGBA16F, w, h, gl::TRUE);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D_MULTISAMPLE,
            self.mc,
            0,
        );
        gl::GenRenderbuffers(1, &mut self.md);
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.md);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 16, gl::DEPTH24_STENCIL8, w, h);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            self.md,
        );
        gl::GenFramebuffers(1, &mut self.hdr);
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.hdr);
        gl::GenTextures(1, &mut self.hc);
        gl::BindTexture(gl::TEXTURE_2D, self.hc);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA16F as i32,
            w,
            h,
            0,
            gl::RGBA,
            gl::FLOAT,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            self.hc,
            0,
        );
        gl::GenRenderbuffers(1, &mut self.hd);
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.hd);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, w, h);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            self.hd,
        );
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0)
    }
}
#[derive(Clone, Copy)]
struct R {
    x: f32,
    z: f32,
    y: f32,
    b: f32,
    h: f32,
    hy: f32,
    e: f32,
    a: f32,
    sa: f32,
}
const POSE: [R; 5] = [
    R {
        x: -8.,
        z: -10.,
        y: 0.,
        b: 0.,
        h: 0.,
        hy: 0.,
        e: 0.,
        a: 90.,
        sa: 90.,
    },
    R {
        x: 4.,
        z: -7.,
        y: 90.,
        b: 20.,
        h: -10.,
        hy: 30.,
        e: -30.,
        a: 0.,
        sa: 30.,
    },
    R {
        x: 6.,
        z: -1.,
        y: 30.,
        b: 0.,
        h: 30.,
        hy: 0.,
        e: 30.,
        a: 90.,
        sa: 90.,
    },
    R {
        x: 0.,
        z: 10.,
        y: 220.,
        b: 0.,
        h: 0.,
        hy: -40.,
        e: -20.,
        a: 0.,
        sa: 0.,
    },
    R {
        x: -10.,
        z: 0.,
        y: -90.,
        b: 0.,
        h: -20.,
        hy: 0.,
        e: 0.,
        a: 90.,
        sa: 60.,
    },
];
fn lr(a: R, b: R, t: f32) -> R {
    let q = |x, y| x + (y - x) * t;
    R {
        x: q(a.x, b.x),
        z: q(a.z, b.z),
        y: q(a.y, b.y),
        b: q(a.b, b.b),
        h: q(a.h, b.h),
        hy: q(a.hy, b.hy),
        e: q(a.e, b.e),
        a: q(a.a, b.a),
        sa: q(a.sa, b.sa),
    }
}
unsafe fn scene(
    p: u32,
    a: &A,
    vp: M,
    c: &Camera,
    r: R,
    day: V,
    spot: V,
    dir: V,
    w: bool,
    time: f32,
) {
    let d = |m, mesh: &Mesh, tx: &Tex, rep| {
        draw(
            p,
            mesh,
            m,
            vp,
            c,
            tx,
            &a.white,
            day,
            spot,
            dir,
            w,
            [0., 0.],
            rep,
        )
    };
    d(
        xform([0., -0.1, 0.], [0.; 3], [30., 0.2, 30.]),
        &a.cube,
        &a.floor,
        [2.5, 2.5],
    );
    d(
        xform([0., 7.5, -15.], [0.; 3], [30., 15., 0.25]),
        &a.cube,
        &a.wall,
        [1., 1.],
    );
    for x in [-10., 0., 10.] {
        for y in [2.5, 7.5, 12.5] {
            if x != 0. || y != 7.5 {
                d(
                    xform([-15., y, x], [0.; 3], [0.25, 5., 10.]),
                    &a.cube,
                    &a.wall,
                    [1., 1.],
                )
            }
        }
    }
    d(
        xform([-14.8, 7.5, 0.], [0., 90., 0.], [9.8, 10., 0.1]),
        &a.cube,
        &a.window,
        [1., 1.],
    );
    draw(
        p,
        &a.plane,
        xform([-14.65, 7.5, 0.], [0., 90., 0.], [9.8, 1., 9.8]),
        vp,
        c,
        &a.snow,
        &a.white,
        day,
        spot,
        dir,
        w,
        [time * 0.5 + time.sin() * 0.6, time * 0.5],
        [1., 1.],
    );
    for (pos, sz, tx) in [
        ([-7.75, 7.5, -14.6], [9., 10.5, 0.3], &a.wood),
        ([6., 9., -14.6], [4.5, 6., 0.3], &a.paint),
        ([10., 0.5, -10.], [2., 1., 1.], &a.marble),
        ([10., 4., -10.], [3.5, 6., 0.4], &a.phone),
        ([0., 0.1, 0.], [5., 0.2, 5.], &a.marble),
    ] {
        d(xform(pos, [0.; 3], sz), &a.cube, tx, [1., 1.])
    }
    d(
        xform([0., 4.2, 0.], [0.; 3], [5., 8., 5.]),
        &a.sphere,
        &a.marble,
        [2., 2.],
    );
    let root = mm(t([r.x, 0., r.z]), ry(r.y));
    let wheel = mm(root, xform([0., 0.8, 0.], [r.b, 0., 90.], [1.6, 0.5, 1.6]));
    d(wheel, &a.sphere, &a.metal, [1., 1.]);
    let body = mm(wheel, xform([0., 1.5, 0.], [0.; 3], [2., 3., 1.]));
    d(body, &a.cube, &a.robot, [1., 1.]);
    let j = mm(body, xform([0., 1.6, 0.], [r.h, 0., 0.], [0.8, 0.8, 0.8]));
    d(j, &a.sphere, &a.metal, [1., 1.]);
    let head = mm(j, xform([0., 0.6, 0.], [0., r.hy, 0.], [3., 1., 3.]));
    d(head, &a.sphere, &a.robot, [1., 1.]);
    d(
        mm(
            head,
            xform([0., 0., 1.25], [r.e, 0., 0.], [0.75, 0.75, 0.25]),
        ),
        &a.sphere,
        &a.robot,
        [1., 1.],
    );
    for (yaw, ang, len) in [(0., r.a, 2.), (-45., r.sa, 1.)] {
        let q = mm(head, mm(ry(yaw), t([0., 0.5, -1.25])));
        d(mm(q, s([0.25, 0.5, 0.25])), &a.cube, &a.robot, [1., 1.]);
        let q = mm(q, xform([0., 0.35, 0.], [ang, 0., 0.], [0.25, 0.25, 0.25]));
        d(q, &a.sphere, &a.metal, [1., 1.]);
        d(
            mm(q, xform([0., len / 2., 0.], [0.; 3], [0.1, len, 0.15])),
            &a.cube,
            &a.metal,
            [1., 1.],
        )
    }
    let base = xform([13., 0., 5.], [0.; 3], [1., 0.2, 1.]);
    d(base, &a.cube, &a.wood, [1., 1.]);
    d(
        mm(base, xform([0., 5., 0.], [0.; 3], [0.2, 10., 0.2])),
        &a.cube,
        &a.wood,
        [1., 4.],
    );
    let arm = mm(base, xform([-1.9, 10.1, 0.], [0., 0., 90.], [0.2, 4., 0.2]));
    d(arm, &a.cube, &a.wood, [1., 4.]);
    d(
        mm(
            arm,
            xform([-1.9, 0., 0.], [time.sin() * 30., 0., 0.], [0.8, 1., 0.8]),
        ),
        &a.cube,
        &a.wood,
        [1., 1.],
    )
}
fn main() {
    let mut g = glfw::init(fail_on_errors!()).unwrap();
    g.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    g.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut win, ev) = g
        .create_window(
            1024,
            768,
            "Museum — Rust / GLFW",
            glfw::WindowMode::Windowed,
        )
        .expect("GLFW window");
    win.make_current();
    win.set_all_polling(true);
    g.set_swap_interval(glfw::SwapInterval::Sync(1));
    unsafe {
        gl::load_with(|s| {
            g.get_proc_address_raw(s)
                .map_or(ptr::null(), |p| p as *const c_void)
        });
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(0.005, 0.005, 0.005, 1.);
        let (p, sky, post) = (pg(VERT, FRAG), pg(SKYV, SKYF), pg(POSTV, POSTF));
        let a = make_a();
        let mut fb = Fbo::new();
        let mut c = Camera::new();
        let (mut r, mut goal) = (POSE[0], POSE[0]);
        let (mut blend, mut wasd, mut wire, mut sphere) = (1., false, false, true);
        let (mut last, zero) = (Instant::now(), Instant::now());
        while !win.should_close() {
            let now = Instant::now();
            let dt = (now - last).as_secs_f32().min(0.1);
            last = now;
            let tm = zero.elapsed().as_secs_f32();
            for (_, e) in glfw::flush_messages(&ev) {
                match e {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        win.set_should_close(true)
                    }
                    WindowEvent::Key(k, _, Action::Press, _) => match k {
                        Key::F1 => wasd = !wasd,
                        Key::F => wire = !wire,
                        Key::B => sphere = !sphere,
                        Key::Num1 => c.preset(1),
                        Key::Num2 => c.preset(2),
                        Key::Num3 => c.preset(3),
                        Key::Num4 => {
                            goal = POSE[0];
                            blend = 0.
                        }
                        Key::Num5 => {
                            goal = POSE[1];
                            blend = 0.
                        }
                        Key::Num6 => {
                            goal = POSE[2];
                            blend = 0.
                        }
                        Key::Num7 => {
                            goal = POSE[3];
                            blend = 0.
                        }
                        Key::Num8 => {
                            goal = POSE[4];
                            blend = 0.
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            if win.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
                let (x, y) = win.get_cursor_pos();
                let (w, h) = win.get_size();
                c.yaw += (x - w as f64 / 2.) as f32 * 0.001;
                c.pitch -= (y - h as f64 / 2.) as f32 * 0.001;
                c.pitch = c.pitch.clamp(-1.5707, 1.5707);
                c.upd();
                win.set_cursor_pos(w as f64 / 2., h as f64 / 2.);
            }
            let down = |k| win.get_key(k) != Action::Release;
            if wasd {
                if down(Key::W) {
                    c.mv(c.f, dt)
                }
                if down(Key::S) {
                    c.mv(c.f.map(|x| -x), dt)
                }
                if down(Key::A) {
                    c.mv(c.r.map(|x| -x), dt)
                }
                if down(Key::D) {
                    c.mv(c.r, dt)
                }
                if down(Key::E) {
                    c.mv(c.u, dt)
                }
                if down(Key::Q) {
                    c.mv(c.u.map(|x| -x), dt)
                }
            } else {
                if down(Key::A) {
                    c.mv(c.f, dt)
                }
                if down(Key::Z) {
                    c.mv(c.f.map(|x| -x), dt)
                }
                if down(Key::Left) {
                    c.mv(c.r.map(|x| -x), dt)
                }
                if down(Key::Right) {
                    c.mv(c.r, dt)
                }
                if down(Key::Up) {
                    c.mv(c.u, dt)
                }
                if down(Key::Down) {
                    c.mv(c.u.map(|x| -x), dt)
                }
            }
            blend = (blend + dt / 3.).min(1.);
            r = lr(r, goal, blend);
            let (w, h) = win.get_framebuffer_size();
            fb.resize(w, h);
            gl::Viewport(0, 0, w, h);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb.msaa);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            let vp = mm(
                proj(w as f32 / h.max(1) as f32),
                look(c.p, add(c.p, c.f), c.u),
            );
            let day = [
                ((tm * 0.4 - 0.4).sin() + 1.2) * 0.5,
                ((tm * 0.4 - 0.3).sin() + 1.2) * 0.5,
                ((tm * 0.4).sin() + 1.2) * 0.5,
            ];
            scene(
                p,
                &a,
                vp,
                &c,
                r,
                day,
                [9.2, 10., 5.],
                [0., -1., 0.],
                wire,
                tm,
            );
            if !wire {
                gl::CullFace(gl::FRONT);
                gl::UseProgram(sky);
                um(sky, "vp", &vp);
                uv(sky, "tint", day);
                gl::Uniform2f(l(sky, "offset"), if sphere { tm * 0.001 } else { 0. }, 0.);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, a.sky.0);
                gl::Uniform1i(l(sky, "tex"), 0);
                a.sphere.draw();
                gl::CullFace(gl::BACK)
            }
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, fb.msaa);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, fb.hdr);
            gl::BlitFramebuffer(0, 0, w, h, 0, 0, w, h, gl::COLOR_BUFFER_BIT, gl::NEAREST);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Disable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(post);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, fb.hc);
            gl::Uniform1i(l(post, "tex"), 0);
            fb.quad.draw();
            gl::Enable(gl::DEPTH_TEST);
            win.swap_buffers();
            g.poll_events();
        }
    }
}
