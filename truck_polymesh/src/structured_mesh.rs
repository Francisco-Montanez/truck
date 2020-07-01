use geometry::{Vector3};
use crate::errors::Error;
use crate::{PolygonMesh, StructuredMesh};

impl StructuredMesh {
    pub fn new(
        positions: Vec<Vec<Vector3>>,
        (u_div, v_div): (Vec<f64>, Vec<f64>),
        normals: Vec<Vec<Vector3>>,
    ) -> StructuredMesh
    {
        if positions.len() != u_div.len() || normals.len() != u_div.len() {
            panic!("{}", Error::DifferentLengthArrays);
        }
        for arr in &positions {
            if arr.len() != v_div.len() {
                panic!("{}", Error::IrregularArray);
            }
        }
        for arr in &normals {
            if arr.len() != v_div.len() {
                panic!("{}", Error::IrregularArray);
            }
        }
        for i in 1..u_div.len() {
            if u_div[i - 1] > u_div[i] {
                panic!("{}", Error::UnsortedDivision);
            }
        }
        for i in 1..v_div.len() {
            if v_div[i - 1] > v_div[i] {
                panic!("{}", Error::UnsortedDivision);
            }
        }
        StructuredMesh {
            positions: positions,
            uv_division: (u_div, v_div),
            normals: normals,
        }
    }

    pub fn new_unchecked(
        positions: Vec<Vec<Vector3>>,
        (u_div, v_div): (Vec<f64>, Vec<f64>),
        normals: Vec<Vec<Vector3>>,
    ) -> StructuredMesh
    {
        StructuredMesh {
            positions: positions,
            uv_division: (u_div, v_div),
            normals: normals,
        }
    }
    pub fn by_positions(positions: Vec<Vec<Vector3>>) -> StructuredMesh {
        for arr in &positions {
            if arr.len() != positions[0].len() {
                panic!("{}", Error::IrregularArray);
            }
        }

        StructuredMesh {
            positions: positions,
            uv_division: (Vec::new(), Vec::new()),
            normals: Vec::new(),
        }
    }

    pub fn destruct(self) -> PolygonMesh {
        let mut mesh = PolygonMesh::default();
        let m = self.positions.len();
        let n = self.positions[0].len();
        mesh.positions = self
            .positions
            .iter()
            .flat_map(|vec| vec.iter())
            .map(|x| x.clone())
            .collect();
        mesh.uv_coords = self.uv_division.0.iter()
            .flat_map(|u| {
                self.uv_division.1.iter().map(move |v| {
                    vector!(*u, *v)
                })
            })
            .collect();
        mesh.normals = self
            .normals
            .into_iter()
            .flat_map(|vec| vec.into_iter())
            .collect();
        for i in 1..m {
            for j in 1..n {
                let face = [
                    [(i - 1) * n + j - 1; 3],
                    [i * n + j - 1; 3],
                    [i * n + j; 3],
                    [(i - 1) * n + j; 3],
                ];
                mesh.quad_faces.push(face);
            }
        }
        mesh
    }
}
