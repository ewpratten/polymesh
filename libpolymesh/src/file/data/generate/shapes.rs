use super::super::{
    mesh::{
        PolyMesh,
        PolyColor
    },
    vector::PolyVec
};

pub enum AxisDirection {
    PositiveX,
    PositiveY,
    PositiveZ,
    NegativeX,
    NegativeY,
    NegativeZ
}

pub fn gen_aaplane(direction: AxisDirection, width: f32, height: f32, color: PolyColor, emission: Option<f32>, albedo: Option<f32>) -> PolyMesh {

    // Create triangles list
    let mut triangles = Vec::new();

    match direction {
        AxisDirection::PositiveX => {
            triangles.push([
                PolyVec { x: 0.0, y: -height, z: -width },
                PolyVec { x: 0.0, y: height, z: width },
                PolyVec { x: 0.0, y: -height, z: width }
            ]);
            triangles.push([
                PolyVec { x: 0.0, y: -height, z: -width },
                PolyVec { x: 0.0, y: height, z: -width },
                PolyVec { x: 0.0, y: height, z: width }
            ]);
        },
        AxisDirection::PositiveY => {
            triangles.push([
                PolyVec { x: -width, y: 0.0, z: -height },
                PolyVec { x: -width, y: 0.0, z: height },
                PolyVec { x: width, y: 0.0, z: height }
            ]);
            triangles.push([
                PolyVec { x: -width, y: 0.0, z: -height },
                PolyVec { x: width, y: 0.0, z: height },
                PolyVec { x: width, y: 0.0, z: -height }
            ]);
        },
        AxisDirection::PositiveZ => {
            triangles.push([
                PolyVec { x: -width, y: -height, z:0.0  },
                PolyVec { x: width, y: height, z: 0.0 },
                PolyVec { x: -width, y: height, z: 0.0 }
            ]);
            triangles.push([
                PolyVec { x: -width, y: -height, z: 0.0 },
                PolyVec { x: width, y: -height, z: 0.0 },
                PolyVec { x: width, y: height, z: 0.0 }
            ]);
        },

        AxisDirection::NegativeX => {
            triangles.push([
                PolyVec { x: 0.0, y: -height, z: -width },
                PolyVec { x: 0.0, y: -height, z: width },
                PolyVec { x: 0.0, y: height, z: width }
            ]);
            triangles.push([
                PolyVec { x: 0.0, y: -height, z: -width },
                PolyVec { x: 0.0, y: height, z: width },
                PolyVec { x: 0.0, y: height, z: -width }
            ]);
        },
        AxisDirection::NegativeY => {
            triangles.push([
                PolyVec { x: -width, y: 0.0, z: -height },
                PolyVec { x: width, y: 0.0, z: height },
                PolyVec { x: -width, y: 0.0, z: height }
            ]);
            triangles.push([
                PolyVec { x: -width, y: 0.0, z: -height },
                PolyVec { x: width, y: 0.0, z: -height },
                PolyVec { x: width, y: 0.0, z: height }
            ]);
        },
        AxisDirection::NegativeZ => {
            triangles.push([
                PolyVec { x: -width, y: -height, z:0.0  },
                PolyVec { x: -width, y: height, z: 0.0 },
                PolyVec { x: width, y: height, z: 0.0 }
            ]);
            triangles.push([
                PolyVec { x: -width, y: -height, z: 0.0 },
                PolyVec { x: width, y: height, z: 0.0 },
                PolyVec { x: width, y: -height, z: 0.0 }
            ]);
        }
    }

    // Build PolyMesh
    PolyMesh {
        triangles: triangles,
        color: color,
        emission: emission,
        albedo: albedo
    }

}

