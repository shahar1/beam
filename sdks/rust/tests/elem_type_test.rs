use apache_beam::elem_types::ElemType;

#[test]
fn elem_type_object_safety() {
    fn _f(_e: &dyn ElemType) {}
}
