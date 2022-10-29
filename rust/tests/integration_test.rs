{% if crate_type == "lib" %}#[test]
fn test_add() {
    assert_eq!({{crate_name}}::add(3, 2), 5);
}
{% else %}#[test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
{% endif %}
