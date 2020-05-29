use crate::types::{Event, RepoCitation};

#[derive(Debug)]
pub struct Source {
    pub xref: Option<String>,
    pub data: SourceData,
    pub abbreviation: Option<String>,
    pub title: Option<String>,
    repo_citations: Vec<RepoCitation>,
}

impl Source {
    pub fn new(xref: Option<String>) -> Source {
        Source {
            xref,
            data: SourceData {
                events: Vec::new(),
                agency: None,
            },
            abbreviation: None,
            title: None,
            repo_citations: Vec::new(),
        }
    }

    pub fn add_repo_citation(&mut self, citation: RepoCitation) {
        self.repo_citations.push(citation);
    }
}

#[derive(Debug)]
pub struct SourceData {
    events: Vec<Event>,
    pub agency: Option<String>,
}

impl SourceData {
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}