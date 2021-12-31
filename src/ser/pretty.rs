pub(crate) struct Pretty;

impl crate::visit_mut::VisitMut for Pretty {
    fn visit_document_mut(&mut self, node: &mut crate::Document) {
        crate::visit_mut::visit_document_mut(self, node);
        if let Some((_, first)) = node.iter_mut().next() {
            remove_table_prefix(first);
        }
    }

    fn visit_item_mut(&mut self, node: &mut crate::Item) {
        node.make_item();

        crate::visit_mut::visit_item_mut(self, node);
    }

    fn visit_table_mut(&mut self, node: &mut crate::Table) {
        node.decor_mut().clear();
        node.set_implicit(true);

        crate::visit_mut::visit_table_mut(self, node);
    }

    fn visit_value_mut(&mut self, node: &mut crate::Value) {
        node.decor_mut().clear();

        crate::visit_mut::visit_value_mut(self, node);
    }

    fn visit_array_mut(&mut self, node: &mut crate::Array) {
        crate::visit_mut::visit_array_mut(self, node);

        if (0..=1).contains(&node.len()) {
            node.set_trailing("");
            node.set_trailing_comma(false);
        } else {
            for item in node.iter_mut() {
                item.decor_mut().set_prefix("\n    ");
            }
            node.set_trailing("\n");
            node.set_trailing_comma(true);
        }
    }
}

fn remove_table_prefix(node: &mut crate::Item) {
    match node {
        crate::Item::None => {}
        crate::Item::Value(_) => {}
        crate::Item::Table(t) => t.decor_mut().set_prefix(""),
        crate::Item::ArrayOfTables(a) => {
            if let Some(first) = a.values.iter_mut().next() {
                remove_table_prefix(first);
            }
        }
    }
}
