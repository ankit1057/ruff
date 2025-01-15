macro_rules! define_enum {
    (
        enum (id $enum_id:ident, node $enum:ident) {
            $( $variant:ident(id $id:ident, node $node:ident, vec $vec:ident), )+
        }
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq, is_macro::Is)]
        pub enum $enum_id {
            $( $variant($id), )+
        }

        #[derive(Copy, Clone, Debug, PartialEq, is_macro::Is)]
        pub enum $enum<'a> {
            $( $variant(crate::Node<'a, &'a $node>), )+
        }

        impl<'a> crate::Node<'a, $enum_id> {
            #[inline]
            pub fn node(&self) ->  $enum<'a> {
                match self.node {
                    $( $enum_id::$variant(id) => $enum::$variant(self.ast.wrap(&self.ast[id])), )+
                }
            }
        }

        impl ruff_text_size::Ranged for $enum<'_> {
            fn range(&self) -> TextRange {
                match self {
                    $( $enum::$variant(node) => node.range(), )+
                }
            }
        }

        $(
            #[ruff_index::newtype_index]
            pub struct $id;

            impl std::ops::Index<$id> for crate::Ast {
                type Output = $node;
                #[inline]
                fn index(&self, id: $id) -> &$node {
                    &self.$vec[id]
                }
            }

            impl std::ops::IndexMut<$id> for crate::Ast {
                #[inline]
                fn index_mut(&mut self, id: $id) -> &mut $node {
                    &mut self.$vec[id]
                }
            }

            impl<'a> crate::Node<'a, $id> {
                #[inline]
                pub fn node(&self) -> crate::Node<'a, &'a $node> {
                    self.ast.wrap(&self.ast[self.node])
                }
            }

            impl<'a> ruff_text_size::Ranged for $node {
                fn range(&self) -> TextRange {
                    self.range
                }
            }

            impl<'a> ruff_text_size::Ranged for crate::Node<'a, &'a $node> {
                fn range(&self) -> TextRange {
                    self.as_ref().range()
                }
            }
        )+
    }
}

pub(crate) use define_enum;
