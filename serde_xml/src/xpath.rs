use roxmltree::Node;

#[derive(Debug, Clone)]
pub enum XPathStep {
    Element(String),
    Attribute(String),
}

#[derive(Debug, Clone)]
pub struct XPath {
    pub steps: Vec<XPathStep>,
}

impl XPath {
    pub fn parse(path: &str) -> Result<Self, String> {
        let mut steps = Vec::new();

        if path.is_empty() {
            return Ok(XPath { steps });
        }

        let path = path.strip_prefix('/').unwrap_or(path);

        if path.is_empty() {
            return Ok(XPath { steps });
        }

        for part in path.split('/') {
            if part.is_empty() {
                continue;
            }

            if let Some(attr_name) = part.strip_prefix('@') {
                steps.push(XPathStep::Attribute(attr_name.to_string()));
            } else {
                steps.push(XPathStep::Element(part.to_string()));
            }
        }

        Ok(XPath { steps })
    }

    pub fn evaluate_single<'a, 'input>(
        &self,
        node: Node<'a, 'input>,
    ) -> Option<XPathResult<'a, 'input>> {
        let mut current = node;

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                XPathStep::Element(name) => {
                    // For the first step, check if the current node matches
                    // (handles absolute paths like /root/child where node is root)
                    if i == 0 && current.is_element() && current.tag_name().name() == name {
                        // Current node matches, continue to next step
                        continue;
                    }
                    current = current
                        .children()
                        .find(|n| n.is_element() && n.tag_name().name() == name)?;
                }
                XPathStep::Attribute(name) => {
                    let value = current.attribute(name.as_str())?;
                    return Some(XPathResult::Attribute(value));
                }
            }
        }

        Some(XPathResult::Node(current))
    }

    pub fn evaluate_all<'a, 'input>(
        &self,
        node: Node<'a, 'input>,
    ) -> Vec<XPathResult<'a, 'input>> {
        if self.steps.is_empty() {
            return vec![XPathResult::Node(node)];
        }

        let mut current_nodes = vec![node];

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                XPathStep::Element(name) => {
                    // For the first step, check if the current node matches
                    if i == 0 && current_nodes.len() == 1 {
                        let n = current_nodes[0];
                        if n.is_element() && n.tag_name().name() == name {
                            continue;
                        }
                    }

                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        // For the last element step, collect all matching children
                        let mut all_matches = Vec::new();
                        for n in &current_nodes {
                            for child in n.children() {
                                if child.is_element() && child.tag_name().name() == name {
                                    all_matches.push(child);
                                }
                            }
                        }
                        return all_matches.into_iter().map(XPathResult::Node).collect();
                    } else {
                        // For intermediate steps, find first matching child
                        let mut next_nodes = Vec::new();
                        for n in &current_nodes {
                            if let Some(child) = n.children().find(|c| c.is_element() && c.tag_name().name() == name) {
                                next_nodes.push(child);
                            }
                        }
                        current_nodes = next_nodes;
                        if current_nodes.is_empty() {
                            return Vec::new();
                        }
                    }
                }
                XPathStep::Attribute(name) => {
                    let mut results = Vec::new();
                    for n in &current_nodes {
                        if let Some(value) = n.attribute(name.as_str()) {
                            results.push(XPathResult::Attribute(value));
                        }
                    }
                    return results;
                }
            }
        }

        current_nodes.into_iter().map(XPathResult::Node).collect()
    }
}

#[derive(Debug, Clone)]
pub enum XPathResult<'a, 'input> {
    Node(Node<'a, 'input>),
    Attribute(&'a str),
}

impl<'a, 'input> XPathResult<'a, 'input> {
    pub fn as_node(&self) -> Option<Node<'a, 'input>> {
        match self {
            XPathResult::Node(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            XPathResult::Attribute(s) => Some(s),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&'a str> {
        match self {
            XPathResult::Node(n) => {
                n.children()
                    .find(|c| c.is_text())
                    .and_then(|c| c.text())
            }
            XPathResult::Attribute(s) => Some(s),
        }
    }
}
