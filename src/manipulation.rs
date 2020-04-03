use crate::Document;
use crate::Selection;
use html5ever::QualName;
use html5ever::{
    tree_builder::{NoQuirks, TreeBuilderOpts},
    ParseOpts,
};
use markup5ever::local_name;
use markup5ever::{namespace_url, ns};
use tendril::StrTendril;
use tendril::TendrilSink;

macro_rules! parse_html {
    ($html: expr) => {
        html5ever::parse_fragment(
            Document::default(),
            ParseOpts {
                tokenizer: Default::default(),
                tree_builder: TreeBuilderOpts {
                    exact_errors: false,
                    scripting_enabled: true,
                    iframe_srcdoc: false,
                    drop_doctype: true,
                    ignore_missing_rules: false,
                    quirks_mode: NoQuirks,
                },
            },
            QualName::new(None, ns!(html), local_name!("")),
            Vec::new(),
        )
        .one($html)
    };
}

impl<'a> Selection<'a> {
    /// set the html contents of each element in the selection to specified parsed HTML.
    pub fn set_html<T>(&mut self, html: T)
    where
        T: Into<StrTendril>,
    {
        for node in self.nodes() {
            node.remove_children();
        }

        self.append_html(html)
    }

    /// replace_with_html replaces each element in the set of matched elements with
    /// the parsed HTML.
    /// It returns the removed elements.
    ///
    /// This follows the same rules as `append`.
    pub fn replace_with_html<T>(&mut self, html: T)
    where
        T: Into<StrTendril>,
    {
        let dom = parse_html!(html);

        let lasti = self.size() - 1;
        let mut i = 0;

        for node in self.nodes() {
            if i == lasti {
                node.append_prev_siblings_from_another_tree(dom.tree);
                break;
            } else {
                node.append_prev_siblings_from_another_tree(dom.tree.clone());
            }
            i += 1;
        }

        self.remove()
    }

    /// append_thml parses the html and appends it to the set of matched elements.
    pub fn append_html<T>(&mut self, html: T)
    where
        T: Into<StrTendril>,
    {
        let dom = parse_html!(html);
        let lasti = self.size() - 1;
        let mut i = 0;

        for node in self.nodes() {
            if i == lasti {
                node.append_children_from_another_tree(dom.tree);
                break;
            } else {
                node.append_children_from_another_tree(dom.tree.clone());
            }
            i += 1;
        }
    }
}