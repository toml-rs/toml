pub(crate) struct Pretty {
    depth: usize,
    strip_prefix: bool,
}

impl Pretty {
    pub(crate) fn new() -> Self {
        Self {
            depth: 0,
            strip_prefix: true,
        }
    }
}

impl crate::visit_mut::VisitMut for Pretty {
    fn visit_document_mut(&mut self, node: &mut crate::Document) {
        match node.iter().next() {
            Some((_, crate::Item::None)) | Some((_, crate::Item::Value(_))) | None => {
                self.strip_prefix = false;
            }
            Some((_, crate::Item::Table(_))) | Some((_, crate::Item::ArrayOfTables(_))) => {}
        }
        crate::visit_mut::visit_document_mut(self, node);
    }

    fn visit_table_mut(&mut self, node: &mut crate::Table) {
        self.depth += 1;
        let implicit = !node.is_empty();
        if 1 < self.depth {
            node.decor_mut().clear();

            let children = node.get_values();
            let is_visible_std_table = !(implicit && children.is_empty());
            if is_visible_std_table && self.strip_prefix {
                node.decor_mut().set_prefix("");
                self.strip_prefix = false;
            }
        }

        // Empty tables could be semantically meaningful, so make sure they are not implicit
        if implicit {
            node.set_implicit(true);
        }

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

pub(crate) struct MakeItem;

impl crate::visit_mut::VisitMut for MakeItem {
    fn visit_item_mut(&mut self, node: &mut crate::Item) {
        node.make_item();

        crate::visit_mut::visit_item_mut(self, node);
    }
}
