//! Port of `legacy/graphics/model/Mesh.java`.

#![allow(unsafe_op_in_unsafe_fn)]

pub struct Mesh {
    vertex_array_id: u32,
    vertex_buffer_id: u32,
    element_buffer_id: u32,
    vertex_count: i32,
}

impl Mesh {
    pub const VERTEX_COORD_FLOATS: i32 = 3;
    pub const VERTEX_NORMAL_FLOATS: i32 = 3;
    pub const VERTEX_UV_FLOATS: i32 = 2;
    pub const VERTEX_STRIDE: i32 =
        (Self::VERTEX_COORD_FLOATS + Self::VERTEX_NORMAL_FLOATS + Self::VERTEX_UV_FLOATS)
            * std::mem::size_of::<f32>() as i32;

    pub unsafe fn new(vertices: &[f32], indices: &[u32]) -> Self {
        let mut vertex_array_id = 0;
        let mut vertex_buffer_id = 0;
        let mut element_buffer_id = 0;
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
        gl::GenBuffers(1, &mut vertex_buffer_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            Self::VERTEX_STRIDE,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            Self::VERTEX_STRIDE,
            (Self::VERTEX_COORD_FLOATS as usize * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            Self::VERTEX_STRIDE,
            ((Self::VERTEX_COORD_FLOATS + Self::VERTEX_NORMAL_FLOATS) as usize
                * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(2);
        // `Mesh.fillBuffers` in the Java source attaches the EBO only after
        // all three vertex attributes have been declared.
        gl::GenBuffers(1, &mut element_buffer_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(indices) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::BindVertexArray(0);
        Self {
            vertex_array_id,
            vertex_buffer_id,
            element_buffer_id,
            vertex_count: indices.len() as i32,
        }
    }

    pub const fn vertex_array_id(&self) -> u32 {
        self.vertex_array_id
    }
    pub const fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// Bind this mesh's VAO.  Kept separate from drawing so `Model` can
    /// reproduce the Java debug-shader validation point exactly.
    pub(crate) unsafe fn bind(&self) {
        gl::BindVertexArray(self.vertex_array_id);
    }

    /// Issue the indexed triangle draw while this mesh's VAO is bound.
    pub(crate) unsafe fn draw_elements(&self) {
        gl::DrawElements(
            gl::TRIANGLES,
            self.vertex_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }

    pub(crate) unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
    }

    pub unsafe fn render(&self) {
        self.bind();
        self.draw_elements();
        self.unbind();
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.element_buffer_id);
            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}
