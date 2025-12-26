use std::collections::HashMap;
use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection,
};

const DAISYUI_DOCS_CONTENT: &str = include_str!("llms.txt");

// ============================================================================
// DocsCache - Documentation search and retrieval
// ============================================================================

#[derive(Debug, Clone)]
struct DocsCache {
    components: HashMap<String, String>,
    index: HashMap<String, Vec<String>>,
}

impl DocsCache {
    fn load() -> Self {
        let mut components = HashMap::new();
        let mut index: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_component = String::new();
        let mut current_content = String::new();

        for line in DAISYUI_DOCS_CONTENT.lines() {
            if let Some(stripped) = line.strip_prefix("### ") {
                if !current_component.is_empty() {
                    let key = current_component.trim().to_lowercase();
                    components.insert(key.clone(), current_content.trim().to_string());
                    for word in current_content.split_whitespace() {
                        let word_lower = word.to_lowercase();
                        if word_lower.len() > 3 {
                            index.entry(word_lower).or_default().push(key.clone());
                        }
                    }
                }
                current_component = stripped.to_string();
                current_content = format!("{}\n", line);
            } else if !current_component.is_empty() {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }
        if !current_component.is_empty() {
            let key = current_component.trim().to_lowercase();
            components.insert(key.clone(), current_content.trim().to_string());
            for word in current_content.split_whitespace() {
                let word_lower = word.to_lowercase();
                if word_lower.len() > 3 {
                    index.entry(word_lower).or_default().push(key.clone());
                }
            }
        }
        DocsCache { components, index }
    }

    fn list_components(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.components.keys().cloned().collect();
        keys.sort();
        keys
    }

    fn get_doc(&self, name: &str) -> Option<String> {
        if name.is_empty() {
            return None;
        }
        self.components.get(&name.to_lowercase()).cloned()
    }

    fn search(&self, query: &str) -> Vec<(String, String, usize)> {
        if query.is_empty() {
            return Vec::new();
        }
        let query = query.to_lowercase();
        let mut scores: HashMap<String, usize> = HashMap::new();

        for (name, content) in &self.components {
            let mut score = 0;
            if name.contains(&query) {
                score += 100;
            }
            if content.to_lowercase().contains(&query) {
                score += 10;
            }
            for word in query.split_whitespace() {
                if let Some(matches) = self.index.get(word) {
                    if matches.contains(name) {
                        score += 5;
                    }
                }
            }
            if score > 0 {
                scores.insert(name.clone(), score);
            }
        }

        let mut sorted: Vec<_> = scores.keys().cloned().collect();
        sorted.sort_by(|a, b| scores[b].cmp(&scores[a]).then(a.cmp(b)));
        sorted
            .iter()
            .take(20)
            .filter_map(|k| {
                self.components
                    .get(k)
                    .map(|c| (k.clone(), c.clone(), scores[k]))
            })
            .collect()
    }
}

// ============================================================================
// ConceptEngine - Design concepts
// ============================================================================

#[derive(Debug, Clone)]
struct DesignConcept {
    name: String,
    description: String,
    classes: Vec<String>,
    suggestion: String,
    snippet: String,
}

impl DesignConcept {
    fn to_display(&self) -> String {
        format!(
            "## {}\n\n**Description:** {}\n\n**Classes:** {}\n\n**Suggestion:** {}\n\n```html\n{}\n```",
            self.name,
            self.description,
            self.classes.join(", "),
            self.suggestion,
            self.snippet
        )
    }
}

struct ConceptEngine {
    concepts: HashMap<String, DesignConcept>,
}

impl ConceptEngine {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("glassmorphism".into(), DesignConcept {
            name: "Glassmorphism".into(),
            description: "Frosted glass aesthetic with transparency and blur effects".into(),
            classes: vec!["glass".into(), "backdrop-blur".into()],
            suggestion: "Apply glass class to cards and modals for depth".into(),
            snippet: r#"<div class="card glass w-96 shadow-xl"><div class="card-body">Content</div></div>"#.into(),
        });
        m.insert(
            "neumorphism".into(),
            DesignConcept {
                name: "Neumorphism".into(),
                description: "Soft shadows creating extruded surface effect".into(),
                classes: vec!["shadow-lg".into(), "bg-base-200".into()],
                suggestion: "Combine soft shadows with subtle gradients".into(),
                snippet: r#"<button class="btn shadow-lg bg-base-200">Button</button>"#.into(),
            },
        );
        m.insert("darkmode".into(), DesignConcept {
            name: "Dark Mode".into(),
            description: "Dark color scheme with high contrast".into(),
            classes: vec!["bg-base-100".into(), "text-base-content".into()],
            suggestion: "Use data-theme attribute to toggle themes".into(),
            snippet: r#"<html data-theme="dark"><body class="bg-base-100 text-base-content">Content</body></html>"#.into(),
        });
        m.insert(
            "gradient".into(),
            DesignConcept {
                name: "Gradients".into(),
                description: "Color transitions for visual depth".into(),
                classes: vec![
                    "bg-gradient-to-r".into(),
                    "from-primary".into(),
                    "to-secondary".into(),
                ],
                suggestion: "Use gradients sparingly on hero sections and CTAs".into(),
                snippet:
                    r#"<div class="bg-gradient-to-r from-primary to-secondary p-8">Hero</div>"#
                        .into(),
            },
        );
        m.insert(
            "skeleton".into(),
            DesignConcept {
                name: "Skeleton Loading".into(),
                description: "Placeholder UI while content loads".into(),
                classes: vec!["skeleton".into()],
                suggestion: "Use skeleton class on elements for loading state".into(),
                snippet: r#"<div class="skeleton h-32 w-full"></div>"#.into(),
            },
        );
        m.insert(
            "responsive".into(),
            DesignConcept {
                name: "Responsive Design".into(),
                description: "Adapts layout to different screen sizes".into(),
                classes: vec!["sm:".into(), "md:".into(), "lg:".into(), "xl:".into()],
                suggestion: "Use responsive prefixes for breakpoint-specific styles".into(),
                snippet:
                    r#"<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">...</div>"#
                        .into(),
            },
        );
        Self { concepts: m }
    }

    fn get_concept(&self, query: &str) -> Option<&DesignConcept> {
        if query.is_empty() {
            return None;
        }
        self.concepts.get(&query.to_lowercase())
    }

    fn list_concepts(&self) -> Vec<String> {
        let mut v: Vec<String> = self.concepts.keys().cloned().collect();
        v.sort();
        v
    }
}

// ============================================================================
// LayoutEngine - HTML layout generation
// ============================================================================

struct LayoutEngine;

impl LayoutEngine {
    const LAYOUTS: &'static [&'static str] = &[
        "saas",
        "blog",
        "social",
        "kanban",
        "inbox",
        "profile",
        "docs",
        "dashboard",
        "auth",
        "store",
    ];

    fn generate(layout: &str, title: &str) -> String {
        let t = Self::sanitize(title);
        match layout {
            "saas" => Self::saas(&t),
            "blog" => Self::blog(&t),
            "social" => Self::social(&t),
            "kanban" => Self::kanban(&t),
            "inbox" => Self::inbox(&t),
            "profile" => Self::profile(&t),
            "docs" => Self::docs(&t),
            "dashboard" => Self::dashboard(&t),
            "auth" => Self::auth(&t),
            "store" => Self::store(&t),
            _ => Self::saas(&t),
        }
    }

    fn sanitize(text: &str) -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
            .take(100)
            .collect()
    }

    fn saas(t: &str) -> String {
        format!(
            r#"<div class="min-h-screen bg-base-100">
  <div class="navbar bg-base-100 sticky top-0 z-50 border-b border-base-200">
    <div class="flex-1"><a class="btn btn-ghost text-xl font-bold">{t}</a></div>
    <div class="flex-none gap-2">
      <ul class="menu menu-horizontal px-1 hidden sm:flex"><li><a>Features</a></li><li><a>Pricing</a></li></ul>
      <button class="btn btn-primary">Get Started</button>
    </div>
  </div>
  <div class="hero min-h-[80vh] bg-base-200">
    <div class="hero-content text-center">
      <div class="max-w-2xl">
        <h1 class="text-5xl font-extrabold">Build faster with <span class="text-primary">{t}</span></h1>
        <p class="py-6 text-xl text-base-content/80">The ultimate scaffolding engine for modern web apps.</p>
        <button class="btn btn-primary btn-lg">Start Free Trial</button>
      </div>
    </div>
  </div>
  <div class="py-24 bg-base-100">
    <div class="container mx-auto px-4">
      <h2 class="text-3xl font-bold text-center mb-12">Everything you need</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="card bg-base-200 shadow-sm"><div class="card-body"><h3 class="card-title">⚡ Fast</h3><p>Optimized for speed.</p></div></div>
        <div class="card bg-base-200 shadow-sm"><div class="card-body"><h3 class="card-title">🔒 Secure</h3><p>Bank-grade security.</p></div></div>
        <div class="card bg-base-200 shadow-sm"><div class="card-body"><h3 class="card-title">🎨 Themable</h3><p>DaisyUI themes.</p></div></div>
      </div>
    </div>
  </div>
  <footer class="footer p-10 bg-base-300"><nav><header class="footer-title">Company</header><a class="link link-hover">About</a></nav></footer>
</div>"#
        )
    }

    fn blog(t: &str) -> String {
        format!(
            r#"<div class="min-h-screen bg-base-100">
  <div class="navbar bg-base-100 border-b border-base-200">
    <div class="flex-1"><a class="btn btn-ghost text-2xl font-serif">{t}</a></div>
  </div>
  <div class="container mx-auto px-4 py-12">
    <div class="card lg:card-side bg-base-200 shadow-xl mb-16">
      <figure class="lg:w-1/2"><img src="https://picsum.photos/800/600" class="h-full object-cover" /></figure>
      <div class="card-body lg:w-1/2"><h2 class="card-title text-4xl font-serif">Featured Article</h2><p>Exploring cutting-edge patterns.</p><button class="btn btn-primary">Read</button></div>
    </div>
    <div class="grid md:grid-cols-3 gap-8">
      <div class="card bg-base-200"><div class="card-body"><div class="badge badge-ghost mb-2">Tech</div><h3 class="card-title">Post Title</h3><p>Post excerpt...</p></div></div>
    </div>
  </div>
</div>"#
        )
    }

    fn social(t: &str) -> String {
        format!(
            r#"<div class="min-h-screen bg-base-100 flex">
  <div class="w-64 hidden lg:block p-4 border-r border-base-200">
    <div class="text-2xl font-bold text-primary mb-4">{t}</div>
    <ul class="menu"><li><a class="active">🏠 Home</a></li><li><a>🔔 Notifications</a></li><li><a>✉️ Messages</a></li></ul>
    <button class="btn btn-primary w-full mt-8">Post</button>
  </div>
  <div class="flex-1 max-w-2xl border-r border-base-200">
    <div class="sticky top-0 bg-base-100/80 backdrop-blur p-4 border-b font-bold text-xl">Home</div>
    <div class="p-4 border-b"><textarea class="textarea w-full" placeholder="What's happening?"></textarea><button class="btn btn-primary btn-sm float-right">Post</button></div>
    <div class="p-4 border-b hover:bg-base-200/50">
      <div class="flex gap-4"><div class="avatar"><div class="w-12 rounded-full"><img src="https://picsum.photos/100" /></div></div>
      <div><span class="font-bold">User</span> <span class="opacity-50">@user • 2h</span><p class="mt-1">Just shipped! 🚀</p></div></div>
    </div>
  </div>
</div>"#
        )
    }

    fn kanban(t: &str) -> String {
        format!(
            r#"<div class="h-screen flex flex-col bg-base-200">
  <div class="navbar bg-base-100 shadow-sm"><div class="flex-1"><h1 class="text-xl font-bold">{t}</h1></div><button class="btn btn-primary btn-sm">Share</button></div>
  <div class="flex-1 overflow-x-auto p-6">
    <div class="flex gap-6">
      <div class="w-80 shrink-0"><h3 class="font-bold mb-3">To Do <span class="badge badge-sm">3</span></h3>
        <div class="card bg-base-100 p-4 mb-2"><div class="badge badge-warning mb-2">Design</div><p class="font-semibold">Create mockups</p></div>
        <button class="btn btn-ghost btn-block">+ Add Task</button>
      </div>
      <div class="w-80 shrink-0"><h3 class="font-bold mb-3">In Progress <span class="badge badge-sm">1</span></h3>
        <div class="card bg-base-100 p-4"><div class="badge badge-info mb-2">Dev</div><p class="font-semibold">Implement Auth</p><progress class="progress progress-primary mt-2" value="40" max="100"></progress></div>
      </div>
      <div class="w-80 shrink-0"><h3 class="font-bold mb-3">Done <span class="badge badge-sm">2</span></h3>
        <div class="card bg-base-100 p-4 opacity-60"><p class="line-through">Setup Repo</p></div>
      </div>
    </div>
  </div>
</div>"#
        )
    }

    fn inbox(t: &str) -> String {
        format!(
            r#"<div class="h-screen flex bg-base-100">
  <div class="w-64 border-r flex flex-col">
    <div class="p-4 font-bold text-xl"><div class="badge badge-primary badge-lg mr-2">M</div>{t}</div>
    <button class="btn btn-primary mx-4">✏️ Compose</button>
    <ul class="menu flex-1 p-2"><li><a class="active">Inbox <span class="badge">4</span></a></li><li><a>Sent</a></li><li><a>Drafts</a></li></ul>
  </div>
  <div class="w-80 border-r overflow-y-auto">
    <input class="input input-bordered w-full m-2" placeholder="Search" style="width:calc(100%-1rem)" />
    <div class="p-4 hover:bg-base-200 cursor-pointer border-b"><span class="font-bold">Sender</span><div class="font-semibold truncate">Subject line</div><div class="text-sm opacity-60 truncate">Preview text...</div></div>
  </div>
  <div class="flex-1 flex flex-col">
    <div class="p-6 border-b"><h2 class="text-2xl font-bold">Email Subject</h2><div class="mt-2 text-sm">From: <span class="font-bold">sender@example.com</span></div></div>
    <div class="p-6 flex-1"><p>Email content goes here...</p></div>
  </div>
</div>"#
        )
    }

    fn profile(t: &str) -> String {
        format!(
            r#"<div class="min-h-screen bg-base-200 p-4 md:p-8">
  <div class="max-w-4xl mx-auto">
    <h1 class="text-3xl font-bold mb-8">{t}</h1>
    <div class="flex flex-col md:flex-row gap-6">
      <ul class="menu bg-base-100 rounded-box w-full md:w-64 shadow-sm"><li><a class="active">General</a></li><li><a>Account</a></li><li><a>Notifications</a></li><li><a class="text-error">Danger Zone</a></li></ul>
      <div class="flex-1 card bg-base-100 shadow-sm">
        <div class="card-body">
          <h2 class="card-title mb-4">Profile Information</h2>
          <div class="flex items-center gap-4 mb-6"><div class="avatar placeholder"><div class="bg-neutral text-neutral-content rounded-full w-24"><span class="text-3xl">U</span></div></div><button class="btn btn-sm btn-outline">Change Avatar</button></div>
          <div class="form-control mb-4"><label class="label">Name</label><input class="input input-bordered" value="User Name" /></div>
          <div class="form-control mb-4"><label class="label">Email</label><input class="input input-bordered" value="user@example.com" /></div>
          <div class="form-control mb-4"><label class="label">Bio</label><textarea class="textarea textarea-bordered">Bio here...</textarea></div>
          <button class="btn btn-primary">Save Changes</button>
        </div>
      </div>
    </div>
  </div>
</div>"#
        )
    }

    fn docs(t: &str) -> String {
        format!(
            r#"<div class="drawer lg:drawer-open">
  <input id="docs-drawer" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content">
    <div class="navbar bg-base-100 border-b lg:hidden"><label for="docs-drawer" class="btn btn-ghost">☰</label><span class="font-bold">{t}</span></div>
    <div class="p-8 max-w-4xl mx-auto">
      <div class="text-sm breadcrumbs mb-4"><ul><li><a>Docs</a></li><li>Installation</li></ul></div>
      <h1 class="text-4xl font-bold mb-6">Installation</h1>
      <p class="mb-4 text-lg">Get started in minutes.</p>
      <div class="mockup-code mb-6"><pre data-prefix="$"><code>npm install package-name</code></pre></div>
      <h2 class="text-2xl font-bold mt-8 mb-4">Configuration</h2>
      <p>Add to your config file.</p>
      <div class="alert alert-info mt-8"><span>Requires Node.js 18+</span></div>
    </div>
  </div>
  <div class="drawer-side border-r"><label for="docs-drawer" class="drawer-overlay"></label>
    <ul class="menu p-4 w-80 min-h-full bg-base-100"><li class="menu-title">{t} Docs</li><li><a class="active">Installation</a></li><li><a>Usage</a></li><li><a>Components</a></li></ul>
  </div>
</div>"#
        )
    }

    fn dashboard(t: &str) -> String {
        format!(
            r#"<div class="drawer lg:drawer-open">
  <input id="dash-drawer" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col">
    <div class="navbar bg-base-300"><div class="lg:hidden"><label for="dash-drawer" class="btn btn-ghost">☰</label></div><div class="flex-1 font-bold text-xl px-4">{t}</div></div>
    <div class="p-6">
      <h2 class="text-2xl font-bold mb-6">Dashboard</h2>
      <div class="stats shadow mb-6 w-full">
        <div class="stat"><div class="stat-title">Users</div><div class="stat-value">31K</div><div class="stat-desc">↗︎ 22%</div></div>
        <div class="stat"><div class="stat-title">Revenue</div><div class="stat-value">$12.5K</div><div class="stat-desc">↗︎ 14%</div></div>
        <div class="stat"><div class="stat-title">Orders</div><div class="stat-value">1,234</div><div class="stat-desc">↘︎ 3%</div></div>
      </div>
      <div class="card bg-base-100 shadow"><div class="card-body"><h3 class="card-title">Recent Activity</h3><p>Activity items go here...</p></div></div>
    </div>
  </div>
  <div class="drawer-side"><label for="dash-drawer" class="drawer-overlay"></label>
    <ul class="menu p-4 w-80 min-h-full bg-base-200"><li class="menu-title">Menu</li><li><a class="active">Overview</a></li><li><a>Analytics</a></li><li><a>Settings</a></li></ul>
  </div>
</div>"#
        )
    }

    fn auth(t: &str) -> String {
        format!(
            r#"<div class="hero min-h-screen bg-base-200">
  <div class="card w-full max-w-sm shadow-2xl bg-base-100">
    <form class="card-body">
      <h1 class="text-2xl font-bold text-center">{t}</h1>
      <div class="form-control"><label class="label"><span class="label-text">Email</span></label><input type="email" class="input input-bordered" required /></div>
      <div class="form-control"><label class="label"><span class="label-text">Password</span></label><input type="password" class="input input-bordered" required /><label class="label"><a class="label-text-alt link link-hover">Forgot password?</a></label></div>
      <div class="form-control mt-6"><button class="btn btn-primary">Login</button></div>
      <div class="divider">OR</div>
      <button class="btn btn-outline">Sign up</button>
    </form>
  </div>
</div>"#
        )
    }

    fn store(t: &str) -> String {
        format!(
            r#"<div class="min-h-screen bg-base-100">
  <div class="navbar bg-base-100 border-b"><div class="flex-1"><a class="btn btn-ghost text-xl">{t}</a></div>
    <div class="flex-none"><button class="btn btn-ghost btn-circle"><span class="indicator"><svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg><span class="badge badge-sm indicator-item">3</span></span></button></div>
  </div>
  <div class="hero bg-base-200 py-16"><div class="hero-content text-center"><div><h1 class="text-5xl font-bold">{t}</h1><p class="py-6">Discover amazing products</p><button class="btn btn-primary">Shop Now</button></div></div></div>
  <div class="container mx-auto p-8">
    <h2 class="text-2xl font-bold mb-6">Featured Products</h2>
    <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-6">
      <div class="card bg-base-100 shadow"><figure><img src="https://picsum.photos/400/300" /></figure><div class="card-body"><h3 class="card-title">Product</h3><p>$99.00</p><button class="btn btn-primary btn-sm">Add to Cart</button></div></div>
    </div>
  </div>
</div>"#
        )
    }
}

// ============================================================================
// Extension State
// ============================================================================

struct DaisyDaysExtension {
    docs: DocsCache,
    concepts: ConceptEngine,
}

impl zed::Extension for DaisyDaysExtension {
    fn new() -> Self {
        Self {
            docs: DocsCache::load(),
            concepts: ConceptEngine::new(),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "daisy-search" => {
                let query = args.join(" ");
                if query.is_empty() {
                    return Err("Please provide a search query".into());
                }
                let results = self.docs.search(&query);
                if results.is_empty() {
                    return Ok(SlashCommandOutput {
                        text: format!("No results found for '{}'", query),
                        sections: vec![],
                    });
                }
                let text = results
                    .iter()
                    .map(|(name, _, score)| format!("- **{}** (score: {})", name, score))
                    .collect::<Vec<_>>()
                    .join("\n");
                let output = format!("## Search Results for '{}'\n\n{}", query, text);
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output.len()).into(),
                        label: "Search Results".into(),
                    }],
                    text: output,
                })
            }
            "daisy-doc" => {
                let name = args.join(" ");
                if name.is_empty() {
                    return Err("Please provide a component name".into());
                }
                match self.docs.get_doc(&name) {
                    Some(doc) => Ok(SlashCommandOutput {
                        sections: vec![SlashCommandOutputSection {
                            range: (0..doc.len()).into(),
                            label: format!("Doc: {}", name),
                        }],
                        text: doc,
                    }),
                    None => Err(format!("Documentation not found for '{}'", name)),
                }
            }
            "daisy-components" => {
                let components = self.docs.list_components();
                let text = format!("## DaisyUI Components\n\n{}", components.join(", "));
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Components List".into(),
                    }],
                    text,
                })
            }
            "daisy-concept" => {
                let name = args.join(" ");
                if name.is_empty() {
                    return Err("Please provide a concept name".into());
                }
                match self.concepts.get_concept(&name) {
                    Some(c) => {
                        let text = c.to_display();
                        Ok(SlashCommandOutput {
                            sections: vec![SlashCommandOutputSection {
                                range: (0..text.len()).into(),
                                label: format!("Concept: {}", c.name),
                            }],
                            text,
                        })
                    }
                    None => Err(format!(
                        "Concept '{}' not found. Available: {}",
                        name,
                        self.concepts.list_concepts().join(", ")
                    )),
                }
            }
            "daisy-concepts" => {
                let concepts = self.concepts.list_concepts();
                let text = format!("## Design Concepts\n\n{}", concepts.join(", "));
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Concepts List".into(),
                    }],
                    text,
                })
            }
            "daisy-layout" => {
                let layout = args.first().map(|s| s.as_str()).unwrap_or("saas");
                let title = if args.len() > 1 {
                    args[1..].join(" ")
                } else {
                    "My App".into()
                };
                let html = LayoutEngine::generate(layout, &title);
                let text = format!("## Generated {} Layout\n\n```html\n{}\n```", layout, html);
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Layout: {}", layout),
                    }],
                    text,
                })
            }
            "daisy-layouts" => {
                let layouts = LayoutEngine::LAYOUTS.join(", ");
                let text = format!("## Available Layouts\n\n{}", layouts);
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Layouts List".into(),
                    }],
                    text,
                })
            }
            cmd => Err(format!("Unknown command: {}", cmd)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "daisy-layout" => Ok(LayoutEngine::LAYOUTS
                .iter()
                .map(|l| SlashCommandArgumentCompletion {
                    label: l.to_string(),
                    new_text: l.to_string(),
                    run_command: true,
                })
                .collect()),
            "daisy-concept" => Ok(self
                .concepts
                .list_concepts()
                .iter()
                .map(|c| SlashCommandArgumentCompletion {
                    label: c.clone(),
                    new_text: c.clone(),
                    run_command: true,
                })
                .collect()),
            "daisy-doc" => Ok(self
                .docs
                .list_components()
                .iter()
                .take(20)
                .map(|c| SlashCommandArgumentCompletion {
                    label: c.clone(),
                    new_text: c.clone(),
                    run_command: true,
                })
                .collect()),
            _ => Ok(vec![]),
        }
    }
}

zed::register_extension!(DaisyDaysExtension);
