use alloc::vec::Vec;
use core::ops::Range;

use kstring::KString;

use crate::FxHashMap;
use crate::custom_cmds::CustomCmds;

#[derive(Debug, Default)]
pub(crate) struct GlobalState {
    /// This is used for numbering equations in the document.
    pub(crate) equation_count: usize,
    /// This is used for resolving references to equations in the document. The keys are the labels
    /// defined in the document, and the values are the corresponding equation numbers (as strings).
    pub(crate) label_map: FxHashMap<KString, KString>,
    /// The commands which the document defines for itself, with `\newcommand`, when the
    /// conversion runs in the global group. They stay defined for the snippets which follow.
    pub(crate) custom_cmds: CustomCmds,
    /// Every `\label`, in order, with where it is written and the tag it was given, if its
    /// equation has a number. Unlike `label_map`, this keeps a label which is given twice.
    pub(crate) labels: Vec<(KString, Range<usize>, Option<KString>)>,
    /// Every `\eqref`, in order, with where it is written.
    pub(crate) references: Vec<(KString, Range<usize>)>,
}

impl GlobalState {
    /// Records the tag of the equation which `label` names.
    pub(crate) fn tag_label(&mut self, label: &str, tag: &str) {
        self.label_map
            .insert(KString::from_ref(label), KString::from_ref(tag));
        if let Some((_, _, t)) = self
            .labels
            .iter_mut()
            .rev()
            .find(|(name, _, t)| name.as_str() == label && t.is_none())
        {
            *t = Some(KString::from_ref(tag));
        }
    }
}
