//! Port of `legacy/graphics/model/MeshLoader.java`.

use super::{Mesh, MeshLibrary};
use crate::gmaths::{Vec2, Vec3};
use std::{collections::HashMap, fs, path::Path, rc::Rc};

#[derive(Debug)]
pub enum MeshLoadError {
    Io(std::io::Error),
    InvalidFace(String),
    MissingIndex(String),
    InvalidNumber(String),
}

impl From<std::io::Error> for MeshLoadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub struct MeshLoader;

impl MeshLoader {
    pub unsafe fn load(
        mesh_library: &mut MeshLibrary,
        filename: impl AsRef<Path>,
    ) -> Result<Rc<Mesh>, MeshLoadError> {
        let filename = filename.as_ref();
        println!("Loading mesh - {}", filename.display());
        let source = fs::read_to_string(filename)?;
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut texture_coordinates = Vec::new();
        let mut vertex_data = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_map = HashMap::<String, u32>::new();

        for line in source.lines() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("v") => positions.push(Self::vec3(words)?),
                Some("vn") => normals.push(Self::vec3(words)?),
                Some("vt") => texture_coordinates.push(Self::vec2(words)?),
                Some("f") => {
                    let face: Vec<_> = words.collect();
                    // `MeshLoader.java` explicitly consumes only items 1..3:
                    // a quad therefore contributes its first triangle instead
                    // of being triangulated or rejected.  Preserve that
                    // deliberately limited parser behavior here.
                    if face.len() < 3 {
                        continue;
                    }
                    for label in face.into_iter().take(3) {
                        if let Some(index) = vertex_map.get(label) {
                            indices.push(*index);
                            continue;
                        }
                        let values: Vec<_> = label.split('/').collect();
                        if values.len() != 3 {
                            return Err(MeshLoadError::InvalidFace(label.into()));
                        }
                        let position = positions
                            .get(Self::index(values[0], positions.len())?)
                            .ok_or_else(|| MeshLoadError::MissingIndex(label.into()))?;
                        let uv = texture_coordinates
                            .get(Self::index(values[1], texture_coordinates.len())?)
                            .ok_or_else(|| MeshLoadError::MissingIndex(label.into()))?;
                        let normal = normals
                            .get(Self::index(values[2], normals.len())?)
                            .ok_or_else(|| MeshLoadError::MissingIndex(label.into()))?;
                        vertex_data.extend_from_slice(&[
                            position.x, position.y, position.z, normal.x, normal.y, normal.z, uv.x,
                            uv.y,
                        ]);
                        let index = (vertex_data.len() / 8 - 1) as u32;
                        vertex_map.insert(label.into(), index);
                        indices.push(index);
                    }
                }
                _ => {}
            }
        }
        Ok(unsafe { mesh_library.create_mesh(&vertex_data, &indices) })
    }

    fn index(value: &str, len: usize) -> Result<usize, MeshLoadError> {
        let value: isize = value
            .parse()
            .map_err(|_| MeshLoadError::InvalidNumber(value.into()))?;
        let index = if value < 0 {
            len as isize + value
        } else {
            value - 1
        };
        usize::try_from(index).map_err(|_| MeshLoadError::MissingIndex(value.to_string()))
    }

    fn vec2<'a>(mut words: impl Iterator<Item = &'a str>) -> Result<Vec2, MeshLoadError> {
        Ok(Vec2::new(
            Self::number(words.next())?,
            Self::number(words.next())?,
        ))
    }

    fn vec3<'a>(mut words: impl Iterator<Item = &'a str>) -> Result<Vec3, MeshLoadError> {
        Ok(Vec3::new(
            Self::number(words.next())?,
            Self::number(words.next())?,
            Self::number(words.next())?,
        ))
    }

    fn number(value: Option<&str>) -> Result<f32, MeshLoadError> {
        let value = value.ok_or_else(|| MeshLoadError::InvalidNumber("missing value".into()))?;
        value
            .parse()
            .map_err(|_| MeshLoadError::InvalidNumber(value.into()))
    }
}
