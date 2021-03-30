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

    // Construct child faces
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_x_geo".to_string(),
        mesh: Box::new(make_quad(
            all_max_corner - PolyVector::unit_x(),
            xy_max_corner - PolyVector::unit_x(),
            xz_max_corner - PolyVector::unit_x(),
            x_max_corner - PolyVector::unit_x(),
            color
        )),
        translation: Some(all_max_corner)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_y_geo".to_string(),
        mesh: Box::new(make_quad(
            y_max_corner - PolyVector::unit_y(),
            xy_max_corner - PolyVector::unit_y(),
            yz_max_corner - PolyVector::unit_y(),
            all_max_corner - PolyVector::unit_y(),
            color
        )),
        translation: Some(all_max_corner)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/positive_z_geo".to_string(),
        mesh: Box::new(make_quad(
            yz_max_corner - PolyVector::unit_z(),
            all_max_corner - PolyVector::unit_z(),
            z_max_corner - PolyVector::unit_z(),
            xz_max_corner - PolyVector::unit_z(),
            color
        )),
        translation: Some(all_max_corner)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_x_geo".to_string(),
        mesh: Box::new(make_quad(
            y_max_corner + PolyVector::unit_x(),
            yz_max_corner + PolyVector::unit_x(),
            all_min_corner + PolyVector::unit_x(),
            z_max_corner + PolyVector::unit_x(),
            color
        )),
        translation: Some(all_max_corner)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_y_geo".to_string(),
        mesh: Box::new(make_quad(
            z_max_corner + PolyVector::unit_y(),
            xz_max_corner + PolyVector::unit_y(),
            all_min_corner + PolyVector::unit_y(),
            x_max_corner + PolyVector::unit_y(),
            color
        )),
        translation: Some(all_max_corner)
    });
    parent.add_child(TransPolyMeshPtr {
        path: "/faces_gr/negative_z_geo".to_string(),
        mesh: Box::new(make_quad(
            xy_max_corner + PolyVector::unit_z(),
            y_max_corner + PolyVector::unit_z(),
            x_max_corner + PolyVector::unit_z(),
            all_min_corner + PolyVector::unit_z(),
            color
        )),
        translation: Some(all_max_corner)
    });

    return parent;

}