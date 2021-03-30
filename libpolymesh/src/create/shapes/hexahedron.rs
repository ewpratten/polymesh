use crate::common::{
    PolyMesh,
    MeshDef,
    MeshType,
    TransPolyMeshPtr,
    transform::{
        PolyVector,
        PolyColor
    }
};
use super::quad::make_quad;

pub fn make_hexahedron(corner_a: PolyVector, corner_b: PolyVector, color: PolyColor) -> PolyMesh {

    // Create parent mesh
    let mut parent = PolyMesh::new(MeshType::Group, None);
    parent.set_name("Hexahedron".to_string());

    // Sort the corners
    let all_max_corner = PolyVector::max(corner_a, corner_b);
    let all_min_corner = PolyVector::min(corner_a, corner_b);
    let x_max_corner = PolyVector { x: all_max_corner.x, y: all_min_corner.y, z: all_min_corner.z };
    let y_max_corner = PolyVector { x: all_min_corner.x, y: all_max_corner.y, z: all_min_corner.z };
    let z_max_corner = PolyVector { x: all_min_corner.x, y: all_min_corner.y, z: all_max_corner.z };
    let xy_max_corner = PolyVector { x: all_max_corner.x, y: all_max_corner.y, z: all_min_corner.z };
    let xz_max_corner = PolyVector { x: all_max_corner.x, y: all_min_corner.y, z: all_max_corner.z };
    let yz_max_corner = PolyVector { x: all_min_corner.x, y: all_max_corner.y, z: all_max_corner.z };

    // Define transformation vectors
    let positive_x_transform = PolyVector { x: all_max_corner.x, y: 0.0, z: 0.0 };
    let positive_y_transform = PolyVector { x: 0.0, y: all_max_corner.y, z: 0.0 };
    let positive_z_transform = PolyVector { x: 0.0, y: 0.0, z: all_max_corner.y };
    let negative_x_transform = PolyVector { x: all_min_corner.x, y: 0.0, z: 0.0 };
    let negative_y_transform = PolyVector { x: 0.0, y: all_min_corner.y, z: 0.0 };
    let negative_z_transform = PolyVector { x: 0.0, y: 0.0, z: all_min_corner.y };

    // Construct child faces
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_x_geo".to_string(),
        mesh: Box::new(make_quad(
            all_max_corner - positive_x_transform,
            xy_max_corner - positive_x_transform,
            xz_max_corner - positive_x_transform,
            x_max_corner - positive_x_transform,
            color
        )),
        translation: Some(positive_x_transform)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_y_geo".to_string(),
        mesh: Box::new(make_quad(
            y_max_corner - positive_y_transform,
            xy_max_corner - positive_y_transform,
            yz_max_corner - positive_y_transform,
            all_max_corner - positive_y_transform,
            color
        )),
        translation: Some(positive_y_transform)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_z_geo".to_string(),
        mesh: Box::new(make_quad(
            yz_max_corner - positive_z_transform,
            all_max_corner - positive_z_transform,
            z_max_corner - positive_z_transform,
            xz_max_corner - positive_z_transform,
            color
        )),
        translation: Some(positive_z_transform)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_x_geo".to_string(),
        mesh: Box::new(make_quad(
            y_max_corner - negative_x_transform,
            yz_max_corner - negative_x_transform,
            all_min_corner - negative_x_transform,
            z_max_corner - negative_x_transform,
            color
        )),
        translation: Some(negative_x_transform)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_y_geo".to_string(),
        mesh: Box::new(make_quad(
            z_max_corner - negative_y_transform,
            xz_max_corner - negative_y_transform,
            all_min_corner - negative_y_transform,
            x_max_corner - negative_y_transform,
            color
        )),
        translation: Some(negative_y_transform)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_z_geo".to_string(),
        mesh: Box::new(make_quad(
            xy_max_corner - negative_z_transform,
            y_max_corner - negative_z_transform,
            x_max_corner - negative_z_transform,
            all_min_corner - negative_z_transform,
            color
        )),
        translation: Some(negative_z_transform)
    });

    return parent;

}