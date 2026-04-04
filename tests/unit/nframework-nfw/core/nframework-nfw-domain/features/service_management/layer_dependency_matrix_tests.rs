use nframework_nfw_domain::features::service_management::layer_dependency_matrix::LayerDependencyMatrix;

#[test]
fn allows_expected_layer_dependencies() {
    let matrix = LayerDependencyMatrix::standard();

    assert!(matrix.is_allowed_reference("Application", "Domain"));
    assert!(matrix.is_allowed_reference("Infrastructure", "Application"));
    assert!(matrix.is_allowed_reference("Infrastructure", "Domain"));
    assert!(matrix.is_allowed_reference("Api", "Application"));
    assert!(matrix.is_allowed_reference("Api", "Infrastructure"));
}

#[test]
fn rejects_forbidden_layer_dependencies() {
    let matrix = LayerDependencyMatrix::standard();

    assert!(!matrix.is_allowed_reference("Domain", "Application"));
    assert!(!matrix.is_allowed_reference("Application", "Infrastructure"));
    assert!(!matrix.is_allowed_reference("Api", "Domain"));
}

#[test]
fn reports_all_forbidden_edges() {
    let matrix = LayerDependencyMatrix::standard();
    let violations = matrix.validate_edges(&[
        ("Domain".to_owned(), "Application".to_owned()),
        ("Application".to_owned(), "Infrastructure".to_owned()),
        ("Api".to_owned(), "Domain".to_owned()),
    ]);

    assert_eq!(violations.len(), 3);
}
