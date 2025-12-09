use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::Arc;

// Embed docs directly for offline/wasm usage
const DAISYUI_DOCS_CONTENT: &str = include_str!("llms.txt");

#[derive(Debug, Clone)]
struct DocsCache {
    components: HashMap<String, String>,
}

impl DocsCache {
    fn load() -> Self {
        let mut components = HashMap::new();
        let mut current_component = String::new();
        let mut current_content = String::new();

        for line in DAISYUI_DOCS_CONTENT.lines() {
            if let Some(stripped) = line.strip_prefix("### ") {
                if !current_component.is_empty() {
                    components.insert(current_component.trim().to_lowercase(), current_content.trim().to_string());
                }
                current_component = stripped.to_string();
                current_content = String::new();
                current_content.push_str(line);
                current_content.push('\n');
            } else if !current_component.is_empty() {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }
        if !current_component.is_empty() {
             components.insert(current_component.trim().to_lowercase(), current_content.trim().to_string());
        }
        DocsCache { components }
    }

    fn list_components(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.components.keys().cloned().collect();
        keys.sort();
        keys
    }

    fn get_doc(&self, name: &str) -> Option<String> {
        self.components.get(&name.to_lowercase()).cloned()
    }
    
    fn search(&self, query: &str) -> Vec<(String, String)> {
        let query = query.to_lowercase();
        let mut results = Vec::new();
        for (name, content) in &self.components {
            if name.contains(&query) || content.to_lowercase().contains(&query) {
                results.push((name.clone(), content.clone()));
            }
        }
        results.sort_by(|a, b| a.0.cmp(&b.0)); 
        results
    }
}

// Concept Definitions

#[derive(Debug, Clone, Serialize)]
struct DesignConcept {
    name: String,
    description: String,
    classes: Vec<String>,
    suggestion: String,
    snippet: String,
}

struct ConceptEngine {
    concepts: HashMap<String, DesignConcept>,
}

impl ConceptEngine {
    fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("glassmorphism".to_string(), DesignConcept {
            name: "Glassmorphism".to_string(),
            description: "Transparency and blur.".to_string(),
            classes: vec!["glass".to_string(), "backdrop-blur".to_string()],
            suggestion: "Use .glass on cards.".to_string(),
            snippet: r##"<div class="card glass"></div>"##.to_string()
        });
        Self { concepts: m }
    }

    fn get_concept(&self, query: &str) -> Option<&DesignConcept> {
        self.concepts.get(&query.to_lowercase())
    }
    
    fn list_concepts(&self) -> Vec<String> {
        let mut v: Vec<String> = self.concepts.keys().cloned().collect();
        v.sort();
        v
    }
}

// Layout Generation Logic

struct LayoutEngine;

impl LayoutEngine {
    fn generate(layout: &str, title: &str) -> String {
        match layout {
            "saas" => Self::saas_landing(title),
            "blog" => Self::blog_layout(title),
            "social" => Self::social_feed(title),
            "kanban" => Self::kanban_board(title),
            "inbox" => Self::inbox_layout(title),
            "profile" => Self::settings_profile(title),
            "docs" => Self::docs_layout(title),
            "dashboard" => Self::dashboard(title),
            "auth" => Self::auth_page(title),
            "store" => Self::store_page(title),
            _ => Self::saas_landing(title) // Default
        }
    }

    fn saas_landing(title: &str) -> String {
        format!(r##"
<div class="min-h-screen bg-base-100 font-sans">
  <!-- Navbar -->
  <div class="navbar bg-base-100 sticky top-0 z-50 border-b border-base-200">
    <div class="flex-1"><a class="btn btn-ghost text-xl font-bold">{}</a></div>
    <div class="flex-none gap-2">
       <ul class="menu menu-horizontal px-1 hidden sm:flex">
         <li><a>Features</a></li>
         <li><a>Pricing</a></li>
         <li><a>Contact</a></li>
       </ul>
       <button class="btn btn-primary">Get Started</button>
    </div>
  </div>

  <!-- Hero -->
  <div class="hero min-h-[80vh] bg-base-200">
    <div class="hero-content text-center">
      <div class="max-w-2xl">
        <h1 class="text-5xl font-extrabold tracking-tight">Build faster with <span class="text-primary">Daisy Days</span></h1>
        <p class="py-6 text-xl text-base-content/80">The ultimate scaffolding engine for modern web applications. Stop writing boilerplate.</p>
        <button class="btn btn-primary btn-lg">Start Free Trial</button>
        <button class="btn btn-ghost btn-lg ml-2">Read Docs</button>
      </div>
    </div>
  </div>

  <!-- Features Grid -->
  <div class="py-24 bg-base-100">
    <div class="container mx-auto px-4">
      <h2 class="text-3xl font-bold text-center mb-12">Everything you need</h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="card bg-base-200 shadow-sm border border-base-300">
          <div class="card-body">
             <div class="p-3 bg-primary/10 w-fit rounded-lg text-primary mb-2">⚡</div>
             <h3 class="card-title">Lightning Fast</h3>
             <p>Optimized for speed and performance out of the box.</p>
          </div>
        </div>
        <div class="card bg-base-200 shadow-sm border border-base-300">
          <div class="card-body">
             <div class="p-3 bg-primary/10 w-fit rounded-lg text-primary mb-2">🔒</div>
             <h3 class="card-title">Secure by Default</h3>
             <p>Bank-grade security standards applied automatically.</p>
          </div>
        </div>
        <div class="card bg-base-200 shadow-sm border border-base-300">
          <div class="card-body">
             <div class="p-3 bg-primary/10 w-fit rounded-lg text-primary mb-2">🎨</div>
             <h3 class="card-title">Themable</h3>
             <p>Change the look and feel in seconds with DaisyUI themes.</p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Footer -->
  <footer class="footer p-10 bg-base-300 text-base-content">
    <nav>
      <header class="footer-title">Services</header> 
      <a class="link link-hover">Branding</a>
      <a class="link link-hover">Design</a>
    </nav> 
    <nav>
      <header class="footer-title">Company</header> 
      <a class="link link-hover">About us</a>
      <a class="link link-hover">Contact</a>
    </nav> 
    <nav>
      <header class="footer-title">Legal</header> 
      <a class="link link-hover">Terms of use</a>
      <a class="link link-hover">Privacy policy</a>
    </nav>
  </footer>
</div>
"##, title)
    }

    fn blog_layout(title: &str) -> String {
        format!(r##"
<div class="min-h-screen bg-base-100">
  <div class="navbar bg-base-100 border-b border-base-200">
    <div class="container mx-auto">
      <div class="flex-1"><a class="btn btn-ghost text-2xl font-serif">{}</a></div>
      <div class="flex-none"><button class="btn btn-ghost btn-circle"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg></button></div>
    </div>
  </div>

  <div class="container mx-auto px-4 py-12">
    <!-- Featured -->
    <div class="card lg:card-side bg-base-200 shadow-xl mb-16">
      <figure class="lg:w-1/2"><img src="https://img.daisyui.com/images/stock/photo-1494232410401-ad00d5433cfa.jpg" class="h-full object-cover" /></figure>
      <div class="card-body lg:w-1/2 justify-center">
        <h2 class="card-title text-4xl mb-4 font-serif">The Future of AI Design</h2>
        <p class="text-lg">How artificial intelligence is reshaping the way we build interfaces.</p>
        <div class="card-actions justify-start mt-4">
          <button class="btn btn-primary">Read Article</button>
        </div>
      </div>
    </div>

    <div class="flex flex-col lg:flex-row gap-12">
      <!-- Main Content -->
      <div class="lg:w-2/3">
         <h3 class="text-2xl font-bold mb-6 border-b border-base-300 pb-2">Latest Stories</h3>
         <div class="flex flex-col gap-8">
            <!-- Post -->
            <div class="flex gap-6 items-start">
               <img src="https://img.daisyui.com/images/stock/photo-1559181567-c3190ca9959b.jpg" class="w-32 h-32 rounded-xl object-cover" />
               <div>
                  <div class="badge badge-ghost mb-2">Tech</div>
                  <h4 class="text-xl font-bold hover:text-primary cursor-pointer">Rust vs Go in 2025</h4>
                  <p class="text-base-content/70 mt-2">A deep dive into system performance.</p>
                  <div class="text-sm mt-2 opacity-50">Dec 9 • 5 min read</div>
               </div>
            </div>
            <!-- Post -->
             <div class="flex gap-6 items-start">
               <img src="https://img.daisyui.com/images/stock/photo-1601004890684-d8cbf643f5f2.jpg" class="w-32 h-32 rounded-xl object-cover" />
               <div>
                  <div class="badge badge-ghost mb-2">Lifestyle</div>
                  <h4 class="text-xl font-bold hover:text-primary cursor-pointer">Digital Minimalism</h4>
                  <p class="text-base-content/70 mt-2">Reclaiming your attention span.</p>
                  <div class="text-sm mt-2 opacity-50">Dec 8 • 3 min read</div>
               </div>
            </div>
         </div>
      </div>

      <!-- Sidebar -->
      <div class="lg:w-1/3">
         <div class="card bg-base-200 p-6 mb-6">
            <h3 class="font-bold text-lg mb-4">Newsletter</h3>
            <p class="text-sm mb-4">Get the latest posts delivered right to your inbox.</p>
            <div class="join w-full">
              <input class="input input-bordered join-item w-full" placeholder="Email"/>
              <button class="btn btn-primary join-item">Subscribe</button>
            </div>
         </div>
         
         <div class="mb-6">
           <h3 class="font-bold text-lg mb-4">Categories</h3>
           <div class="flex flex-wrap gap-2">
             <div class="badge badge-outline p-3">Technology</div>
             <div class="badge badge-outline p-3">Design</div>
             <div class="badge badge-outline p-3">Culture</div>
             <div class="badge badge-outline p-3">Business</div>
           </div>
         </div>
      </div>
    </div>
  </div>
</div>
"##, title)
    }

    fn social_feed(title: &str) -> String {
        format!(r##"
<div class="min-h-screen bg-base-100 flex justify-center">
  <!-- Left Sidebar -->
  <div class="w-64 hidden lg:block p-4 fixed left-0 top-0 h-screen border-r border-base-200 overflow-y-auto">
    <div class="text-2xl font-bold text-primary p-4 mb-4">{}</div>
    <ul class="menu w-full text-lg">
      <li><a class="active"><svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg> Home</a></li>
      <li><a><svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/></svg> Notifications</a></li>
      <li><a><svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg> Messages</a></li>
      <li><a><svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg> Profile</a></li>
    </ul>
    <button class="btn btn-primary w-full rounded-full mt-8">Post</button>
  </div>

  <!-- Main Feed -->
  <div class="w-full lg:w-[600px] border-r border-l border-base-200 min-h-screen">
    <div class="sticky top-0 bg-base-100/80 backdrop-blur z-20 border-b border-base-200 p-4 font-bold text-xl">Home</div>
    <!-- Composer -->
    <div class="p-4 border-b border-base-200 flex gap-4">
       <div class="avatar"><div class="w-12 rounded-full"><img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" /></div></div>
       <div class="w-full">
         <textarea class="textarea textarea-ghost w-full text-lg resize-none" placeholder="What is happening?"></textarea>
         <div class="flex justify-end"><button class="btn btn-primary btn-sm rounded-full">Tweet</button></div>
       </div>
    </div>
    <!-- Posts -->
    <div class="p-4 border-b border-base-200 hover:bg-base-200/50 cursor-pointer transition">
       <div class="flex gap-4">
         <div class="avatar"><div class="w-12 rounded-full"><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" /></div></div>
         <div>
            <div class="flex gap-2 items-center"><span class="font-bold">Jane Doe</span> <span class="text-sm opacity-50">@janedoe • 2h</span></div>
            <p class="mt-1">Just shipped a new update for my MCP server! Rust is blazing fast. 🦀🚀</p>
            <div class="flex justify-between mt-3 max-w-sm text-sm opacity-60">
               <button class="hover:text-primary">💬 12</button>
               <button class="hover:text-green-500">♻️ 4</button>
               <button class="hover:text-red-500">❤️ 89</button>
            </div>
         </div>
       </div>
    </div>
    <div class="p-4 border-b border-base-200 hover:bg-base-200/50 cursor-pointer transition">
       <div class="flex gap-4">
         <div class="avatar"><div class="w-12 rounded-full"><img src="https://img.daisyui.com/images/stock/photo-1559181567-c3190ca9959b.jpg" /></div></div>
         <div>
            <div class="flex gap-2 items-center"><span class="font-bold">Tech Insider</span> <span class="text-sm opacity-50">@tech • 4h</span></div>
            <p class="mt-1">DaisyUI 5.0 is coming soon. Are you ready?</p>
         </div>
       </div>
    </div>
  </div>

  <!-- Right Sidebar -->
  <div class="hidden xl:block w-80 p-4 fixed right-0 top-0 h-screen">
     <div class="card bg-base-200">
        <div class="card-body p-4">
           <h3 class="font-bold text-lg mb-2">Trends for you</h3>
           <div class="py-2">
             <div class="text-xs opacity-50">Technology</div>
             <div class="font-bold">#RustLang</div>
             <div class="text-xs opacity-50">12K Posts</div>
           </div>
           <div class="py-2">
             <div class="text-xs opacity-50">Design</div>
             <div class="font-bold">#UIUX</div>
             <div class="text-xs opacity-50">8K Posts</div>
           </div>
        </div>
     </div>
  </div>
</div>
"##, title)
    }

    fn kanban_board(title: &str) -> String {
        format!(r##"
<div class="h-screen flex flex-col bg-base-200">
  <div class="navbar bg-base-100 shadow-sm px-4">
    <div class="flex-1"><h1 class="text-xl font-bold">{}</h1></div>
     <div class="flex-none gap-2">
        <div class="avatar-group -space-x-6">
          <div class="avatar"><div class="w-8"><img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg" /></div></div>
          <div class="avatar"><div class="w-8"><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" /></div></div>
          <div class="avatar placeholder"><div class="w-8 bg-neutral text-neutral-content"><span>+2</span></div></div>
        </div>
        <button class="btn btn-primary btn-sm">Share</button>
     </div>
  </div>

  <div class="flex-1 overflow-x-auto p-6">
    <div class="flex gap-6 h-full">
       <!-- Lane: Todo -->
       <div class="w-80 shrink-0 flex flex-col gap-3">
          <div class="flex justify-between items-center px-1">
             <h3 class="font-bold uppercase text-sm opacity-70">To Do</h3>
             <span class="badge badge-sm">3</span>
          </div>
          <div class="card bg-base-100 shadow-sm p-4 cursor-pointer hover:shadow-md">
             <div class="badge badge-warning text-xs mb-2">Design</div>
             <p class="font-semibold">Create high-fidelity mockups</p>
          </div>
          <div class="card bg-base-100 shadow-sm p-4 cursor-pointer hover:shadow-md">
             <p class="font-semibold">Research competitor market</p>
             <div class="mt-3 flex justify-between items-center">
                <div class="avatar w-6 rounded-full"><img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg"/></div>
                <span class="text-xs opacity-50">Dec 12</span>
             </div>
          </div>
          <button class="btn btn-ghost btn-block text-base-content/50">+ Add Task</button>
       </div>

       <!-- Lane: In Progress -->
       <div class="w-80 shrink-0 flex flex-col gap-3">
          <div class="flex justify-between items-center px-1">
             <h3 class="font-bold uppercase text-sm opacity-70">In Progress</h3>
             <span class="badge badge-sm">1</span>
          </div>
          <div class="card bg-base-100 shadow-sm p-4 cursor-pointer hover:shadow-md">
             <div class="badge badge-info text-xs mb-2">Dev</div>
             <p class="font-semibold">Implement Authentication</p>
             <progress class="progress progress-primary w-full mt-2" value="40" max="100"></progress>
          </div>
          <button class="btn btn-ghost btn-block text-base-content/50">+ Add Task</button>
       </div>

       <!-- Lane: Done -->
       <div class="w-80 shrink-0 flex flex-col gap-3">
          <div class="flex justify-between items-center px-1">
             <h3 class="font-bold uppercase text-sm opacity-70">Done</h3>
             <span class="badge badge-sm">2</span>
          </div>
          <div class="card bg-base-100 shadow-sm p-4 opacity-60">
             <p class="font-semibold line-through">Setup Repo</p>
          </div>
       </div>
    </div>
  </div>
</div>
"##, title)
    }

    fn inbox_layout(title: &str) -> String {
        format!(r##"
<div class="h-screen flex bg-base-100">
  <!-- Sidebar -->
  <div class="w-64 border-r border-base-200 flex flex-col">
     <div class="p-4 flex items-center gap-2 font-bold text-xl"><div class="badge badge-primary badge-lg">M</div> {}</div>
     <div class="p-4"><button class="btn btn-primary btn-block gap-2"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" /></svg> Compose</button></div>
     <ul class="menu flex-1 p-2">
       <li><a class="active">Inbox <span class="badge badge-sm bg-base-100">4</span></a></li>
       <li><a>Starred</a></li>
       <li><a>Sent</a></li>
       <li><a>Drafts</a></li>
     </ul>
  </div>

  <!-- List -->
  <div class="w-80 border-r border-base-200 overflow-y-auto">
     <div class="p-4 border-b border-base-200 sticky top-0 bg-base-100 z-10">
        <input type="text" placeholder="Search mail" class="input input-sm input-bordered w-full" />
     </div>
     <div class="divide-y divide-base-200">
        <div class="p-4 hover:bg-base-200 cursor-pointer bg-base-200/50">
           <div class="flex justify-between mb-1"><span class="font-bold">Apple</span> <span class="text-xs opacity-50">10:00 AM</span></div>
           <div class="font-semibold truncate">Your receipt for...</div>
           <div class="text-sm opacity-60 truncate">Thank you for your purchase of...</div>
        </div>
        <div class="p-4 hover:bg-base-200 cursor-pointer">
           <div class="flex justify-between mb-1"><span class="font-bold">Github</span> <span class="text-xs opacity-50">Yesterday</span></div>
           <div class="font-semibold truncate">Security alert</div>
           <div class="text-sm opacity-60 truncate">A new vulnerability was found...</div>
        </div>
     </div>
  </div>

  <!-- View -->
  <div class="flex-1 flex flex-col">
     <div class="p-6 border-b border-base-200 flex justify-between items-center">
        <div>
           <h2 class="text-2xl font-bold">Your receipt for iCloud+</h2>
           <div class="flex gap-2 items-center mt-2">
              <div class="avatar w-8 rounded-full bg-neutral text-neutral-content grid place-items-center">A</div>
              <div class="text-sm"><span class="font-bold">Apple</span> &lt;no-reply@apple.com&gt;</div>
           </div>
        </div>
        <div class="flex gap-2">
           <button class="btn btn-ghost btn-sm">Reply</button>
           <button class="btn btn-ghost btn-sm">Delete</button>
        </div>
     </div>
     <div class="p-8 flex-1 overflow-y-auto">
        <p>Hello Ahmad,</p>
        <p class="mt-4">This email confirms your subscription was renewed successfully.</p>
        <div class="card bg-base-200 max-w-sm mt-8 p-4">
           <div class="flex justify-between font-bold"><span>Total</span> <span>$0.99</span></div>
        </div>
     </div>
  </div>
</div>
"##, title)
    }

    fn settings_profile(title: &str) -> String {
        format!(r##"
<div class="min-h-screen bg-base-200 p-4 md:p-8">
  <div class="max-w-4xl mx-auto">
     <h1 class="text-3xl font-bold mb-8">{}</h1>
     <div class="flex flex-col md:flex-row gap-6">
        <!-- Sidebar -->
        <div class="w-full md:w-64 shrink-0">
           <ul class="menu bg-base-100 rounded-box w-full shadow-sm">
             <li><a class="active">General</a></li>
             <li><a>Account</a></li>
             <li><a>Notifications</a></li>
             <li><a>Billing</a></li>
             <li><a class="text-error">Danger Zone</a></li>
           </ul>
        </div>

        <!-- Content -->
        <div class="flex-1">
           <div class="card bg-base-100 shadow-sm">
             <div class="card-body">
                <h2 class="card-title mb-4">Profile Information</h2>
                <div class="flex items-center gap-4 mb-6">
                   <div class="avatar placeholder">
                      <div class="bg-neutral text-neutral-content rounded-full w-24">
                        <span class="text-3xl">AH</span>
                      </div>
                   </div>
                   <div>
                      <button class="btn btn-sm btn-outline">Change Avatar</button>
                      <button class="btn btn-sm btn-ghost text-error">Remove</button>
                   </div>
                </div>

                <div class="grid gap-4">
                   <div class="form-control">
                      <label class="label">Display Name</label>
                      <input type="text" value="Ahmad Hamdi" class="input input-bordered" />
                   </div>
                   <div class="form-control">
                      <label class="label">Email Address</label>
                      <input type="email" value="ahmad@example.com" class="input input-bordered" />
                   </div>
                   <div class="form-control">
                      <label class="label">Bio</label>
                      <textarea class="textarea textarea-bordered h-24">Just shipping code.</textarea>
                   </div>
                </div>

                <div class="card-actions justify-end mt-6">
                   <button class="btn btn-primary">Save Changes</button>
                </div>
             </div>
           </div>

           <div class="card bg-base-100 shadow-sm mt-6">
             <div class="card-body">
                <h2 class="card-title">Preferences</h2>
                <div class="form-control">
                  <label class="label cursor-pointer justify-start gap-4">
                    <input type="checkbox" class="toggle toggle-primary" checked />
                    <span class="label-text">Enable email notifications</span>
                  </label>
                </div>
             </div>
           </div>
        </div>
     </div>
  </div>
</div>
"##, title)
    }

    fn docs_layout(title: &str) -> String {
        format!(r##"
<div class="drawer lg:drawer-open">
  <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col">
    <!-- Navbar -->
    <div class="navbar bg-base-100 border-b border-base-200 lg:hidden">
      <div class="flex-none">
        <label for="my-drawer-2" class="btn btn-square btn-ghost">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
        </label>
      </div>
      <div class="flex-1 px-2 mx-2 text-xl font-bold">{}</div>
    </div>
    
    <!-- Main Content -->
    <div class="p-8 md:p-12 max-w-4xl mx-auto w-full">
       <div class="text-sm breadcrumbs mb-4">
          <ul><li><a>Docs</a></li><li><a>Getting Started</a></li><li>Installation</li></ul>
       </div>
       <h1 class="text-4xl font-bold mb-6">Installation</h1>
       <p class="mb-4 text-lg">Learn how to get up and running with our library in minutes.</p>
       
       <div class="mockup-code mb-6">
         <pre data-prefix="$"><code>npm install daisy-framework</code></pre>
       </div>

       <h2 class="text-2xl font-bold mt-8 mb-4">Configuration</h2>
       <p class="mb-4">Add the plugin to your config file:</p>
       <p class="mb-4">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
       
       <div class="alert alert-info mt-8">
         <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
         <span>Note: Typically requires Node.js 18+.</span>
       </div>
    </div>
  </div> 
  <div class="drawer-side border-r border-base-200">
    <label for="my-drawer-2" class="drawer-overlay"></label> 
    <ul class="menu p-4 w-80 min-h-full bg-base-100 text-base-content">
      <li class="mb-4 text-xl font-bold px-4">{} Docs</li>
      <li>
        <h2 class="menu-title">Getting Started</h2>
        <ul>
          <li><a class="active">Installation</a></li>
          <li><a>Usage</a></li>
          <li><a>Theming</a></li>
        </ul>
      </li>
      <li>
        <h2 class="menu-title">Components</h2>
        <ul>
          <li><a>Button</a></li>
          <li><a>Card</a></li>
          <li><a>Modal</a></li>
        </ul>
      </li>
    </ul>
  </div>
</div>
"##, title, title)
    }

    fn dashboard(title: &str) -> String {
        format!(r##"<div class="drawer lg:drawer-open"><input id="my-drawer" type="checkbox" class="drawer-toggle" /><div class="drawer-content flex flex-col"><div class="w-full navbar bg-base-300"><div class="flex-none lg:hidden"><label for="my-drawer" class="btn btn-square btn-ghost"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg></label></div><div class="flex-1 px-2 mx-2 text-xl font-bold">{}</div></div><div class="p-6"><h2 class="text-2xl font-bold mb-4">Dashboard</h2></div></div><div class="drawer-side"><label for="my-drawer" class="drawer-overlay"></label><ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content"><li class="menu-title">Menu</li><li><a>Overview</a></li></ul></div></div>"##, title)
    }
    
    fn auth_page(title: &str) -> String {
        format!(r##"<div class="hero min-h-screen bg-base-200"><div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100"><form class="card-body"><h1 class="text-2xl font-bold">{}</h1><div class="form-control"><label class="label"><span class="label-text">Email</span></label><input type="email" class="input input-bordered" required /></div><div class="form-control"><label class="label"><span class="label-text">Password</span></label><input type="password" class="input input-bordered" required /></div><div class="form-control mt-6"><button class="btn btn-primary">Login</button></div></form></div></div>"##, title)
    }
    
    fn store_page(title: &str) -> String {
        format!(r##"<div class="hero min-h-screen bg-base-200"><div class="hero-content text-center"><div class="max-w-md"><h1 class="text-5xl font-bold">{}</h1><button class="btn btn-primary mt-4">Shop Now</button></div></div></div>"##, title)
    }
}

// Prompt Processing Logic

struct IdeaEngine;

impl IdeaEngine {
    fn process_prompt(prompt: &str) -> String {
        let p = prompt.to_lowercase();
        
        let layout = if p.contains("blog") || p.contains("article") || p.contains("news") {
            "blog"
        } else if p.contains("social") || p.contains("twitter") || p.contains("feed") {
            "social"
        } else if p.contains("kanban") || p.contains("trello") || p.contains("board") || p.contains("task") {
            "kanban"
        } else if p.contains("mail") || p.contains("inbox") || p.contains("message") {
            "inbox"
        } else if p.contains("profile") || p.contains("settings") || p.contains("account") {
            "profile"
        } else if p.contains("docs") || p.contains("documentation") || p.contains("wiki") {
            "docs"
        } else if p.contains("saas") || p.contains("startup") || p.contains("landing") {
            "saas"
        } else if p.contains("dashboard") || p.contains("admin") {
            "dashboard"
        } else {
            "saas" // Default modern skeleton
        };

        // Pass to LayoutEngine
        LayoutEngine::generate(layout, "Generated UI")
    }
}


// Legacy Generator Wrappers
// Maintained for backward compatibility

fn generate_dashboard(title: &str, _items: &[String], _style: &str) -> String {
    LayoutEngine::generate("dashboard", title)
}

fn generate_auth(auth_type: &str) -> String {
    LayoutEngine::generate("auth", if auth_type == "login" { "Login" } else { "Sign Up" })
}

fn generate_store(page: &str) -> String {
    LayoutEngine::generate("store", page)
}

fn generate_theme(name: &str, primary: &str, secondary: &str, accent: &str, base: &str) -> String {
    format!(r##"@plugin "daisyui/theme" {{ name: "{}"; --color-primary: {}; --color-secondary: {}; --color-accent: {}; --color-base-100: {}; }}"##, name, primary, secondary, accent, base)
}

fn scaffold_form(title: &str, fields: &[serde_json::Map<String, Value>]) -> String {
    let mut field_html = String::new();
    for f in fields {
        let name = f.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed");
        field_html.push_str(&format!(r##"<div class="form-control"><label class="label"><span class="label-text">{}</span></label><input type="text" name="{}" class="input input-bordered" /></div>"##, name, name));
    }
    format!(r##"<div class="card bg-base-100 w-full max-w-sm shadow-2xl"><form class="card-body"><h2 class="card-title justify-center">{}</h2>{}<div class="form-control mt-6"><button class="btn btn-primary">Submit</button></div></form></div>"##, title, field_html)
}

fn get_script(component: &str) -> String {
    match component {
        "modal" => "document.getElementById('my_modal_1').showModal();".to_string(),
        "drawer" => "document.getElementById('my-drawer').checked = !document.getElementById('my-drawer').checked;".to_string(),
        _ => "// No script".to_string()
    }
}

fn create_chart(chart_type: &str, id: &str) -> String {
    format!(r##"<canvas id="{}"></canvas><script>new Chart(document.getElementById('{}'), {{ type: '{}', data: {{ datasets: [{{ data: [10, 20] }}] }} }});</script>"##, id, id, chart_type)
}

fn create_complex_table(cols: &[String]) -> String {
    let headers = cols.iter().map(|c| format!("<th>{}</th>", c)).collect::<Vec<_>>().join("");
    format!(r##"<table class="table w-full"><thead><tr>{}</tr></thead><tbody><tr><td>Data</td></tr></tbody></table>"##, headers)
}

// Main Server

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// Simple internal handling of requests
fn main() -> Result<()> {
    // Engraving
    eprintln!("Daisy Days - Engraved by Ahmad Hamdi");
    
    // Load docs from memory
    let docs = Arc::new(DocsCache::load());
    let concepts = Arc::new(ConceptEngine::new());
    
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let req_str = line.trim();
                let req_str = req_str.trim_matches('\u{0}'); // sanitize
                if req_str.is_empty() { continue; }
                
                match serde_json::from_str::<JsonRpcRequest>(req_str) {
                    Ok(req) => {
                        let res = handle_request(req, docs.clone(), concepts.clone());
                        if let Ok(res_str) = serde_json::to_string(&res) {
                            println!("{}", res_str);
                        }
                    }
                    Err(e) => {
                         // Fallback for minimal error
                         println!(r#"{{"jsonrpc":"2.0","error":{{"code":-32700,"message":"Parse error: {}"}},"id":null}}"#, e);
                    }
                }
            }
            Err(_) => { break; }
        }
    }
    Ok(())
}

fn handle_request(req: JsonRpcRequest, docs: Arc<DocsCache>, concepts: Arc<ConceptEngine>) -> JsonRpcResponse {
    let id = req.id.clone();
    
    let result = match req.method.as_str() {
        "initialize" => {
             Ok(json!({
                "protocolVersion": "0.1",
                "serverInfo": { "name": "daisy-days", "version": "1.1.0 (Author: Ahmad Hamdi)" },
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "listChanged": false }
                }
            }))
        },
        "notifications/initialized" => Ok(json!("OK")),
        "tools/list" => {
            Ok(json!({
                "tools": [
                    { "name": "daisyui_idea_to_ui", "description": "Turn a prompt into a stunning UI.", "inputSchema": { "type": "object", "properties": { "prompt": { "type": "string" } }, "required": ["prompt"] } },
                    { 
                        "name": "daisyui_scaffold_layout", 
                        "description": "Generate a modern web layout skeleton.", 
                        "inputSchema": { 
                            "type": "object", 
                            "properties": { 
                                "layout": { "type": "string", "enum": ["saas", "blog", "social", "kanban", "inbox", "profile", "docs", "dashboard", "auth"], "description": "Layout type" },
                                "title": { "type": "string" }
                            },
                            "required": ["layout"]
                        } 
                    },
                    { "name": "daisyui_list_components", "description": "List components.", "inputSchema": { "type": "object", "properties": {} } },
                    { "name": "daisyui_get_docs", "description": "Get docs.", "inputSchema": { "type": "object", "properties": { "component": { "type": "string" } }, "required": ["component"] } },
                    { "name": "daisyui_search", "description": "Search docs.", "inputSchema": { "type": "object", "properties": { "query": { "type": "string" } } } },
                    { "name": "daisyui_get_concept", "description": "Get concept.", "inputSchema": { "type": "object", "properties": { "concept": { "type": "string" } } } },
                    { "name": "daisyui_list_concepts", "description": "List concepts.", "inputSchema": { "type": "object", "properties": {} } },
                    { "name": "daisyui_scaffold_dashboard", "description": "Generate Dashboard (Legacy).", "inputSchema": { "type": "object", "properties": { "title": { "type": "string" }, "style": { "type": "string" } } } },
                    { "name": "daisyui_scaffold_auth", "description": "Generate Auth (Legacy).", "inputSchema": { "type": "object", "properties": { "type": { "type": "string" } } } },
                    { "name": "daisyui_scaffold_store", "description": "Generate Store (Legacy).", "inputSchema": { "type": "object", "properties": { "page": { "type": "string" } } } },
                    { "name": "daisyui_create_chart", "description": "Generate Chart.", "inputSchema": { "type": "object", "properties": { "type": { "type": "string" }, "id": { "type": "string" } } } },
                    { "name": "daisyui_create_table", "description": "Generate Table.", "inputSchema": { "type": "object", "properties": { "columns": { "type": "array" } } } },
                    { "name": "daisyui_generate_theme", "description": "Generate Theme.", "inputSchema": { "type": "object", "properties": { "name": { "type": "string" }, "primary": { "type": "string" }, "base": { "type": "string" } } } },
                    { "name": "daisyui_scaffold_form", "description": "Generate Form.", "inputSchema": { "type": "object", "properties": { "title": { "type": "string" }, "fields": { "type": "array" } } } },
                    { "name": "daisyui_get_script", "description": "Get Script.", "inputSchema": { "type": "object", "properties": { "component": { "type": "string" } } } }
                ]
            }))
        },
        "tools/call" => {
            if let Some(params) = req.params {
                let name = params["name"].as_str().unwrap_or("");
                let args = params["arguments"].as_object();

                match name {
                     "daisyui_idea_to_ui" => {
                        let prompt = args.and_then(|a| a.get("prompt")).and_then(|v| v.as_str()).unwrap_or("");
                        let html = IdeaEngine::process_prompt(prompt);
                        Ok(json!({ "content": [{ "type": "text", "text": html }] }))
                     },
                     "daisyui_scaffold_layout" => {
                        let layout = args.and_then(|a| a.get("layout")).and_then(|v| v.as_str()).unwrap_or("saas");
                        let title = args.and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("My App");
                        Ok(json!({ "content": [{ "type": "text", "text": LayoutEngine::generate(layout, title) }] }))
                     },
                     "daisyui_list_components" => Ok(json!({ "content": [{ "type": "text", "text": docs.list_components().join(", ") }] })),
                     "daisyui_get_docs" => {
                        let c = args.and_then(|a| a.get("component")).and_then(|v| v.as_str()).unwrap_or("");
                        Ok(json!({ "content": [{ "type": "text", "text": docs.get_doc(c).unwrap_or("Not found".to_string()) }] }))
                     },
                     "daisyui_search" => {
                         let q = args.and_then(|a| a.get("query")).and_then(|v| v.as_str()).unwrap_or("");
                         Ok(json!({ "content": [{ "type": "text", "text": format!("Found {}", docs.search(q).len()) }] }))
                     },
                     "daisyui_get_concept" => {
                         let c = args.and_then(|a| a.get("concept")).and_then(|v| v.as_str()).unwrap_or("");
                         Ok(json!({ "content": [{ "type": "text", "text": format!("{:?}", concepts.get_concept(c)) }] }))
                     },
                     "daisyui_list_concepts" => Ok(json!({ "content": [{ "type": "text", "text": concepts.list_concepts().join(", ") }] })),
                     "daisyui_scaffold_dashboard" => {
                         let t = args.and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("Dash");
                         Ok(json!({ "content": [{ "type": "text", "text": generate_dashboard(t, &[], "") }] }))
                     },
                     "daisyui_scaffold_auth" => {
                         let t = args.and_then(|a| a.get("type")).and_then(|v| v.as_str()).unwrap_or("login");
                         Ok(json!({ "content": [{ "type": "text", "text": generate_auth(t) }] }))
                     },
                     "daisyui_scaffold_store" => {
                         let p = args.and_then(|a| a.get("page")).and_then(|v| v.as_str()).unwrap_or("home");
                         Ok(json!({ "content": [{ "type": "text", "text": generate_store(p) }] }))
                     },
                     "daisyui_create_chart" => {
                         let t = args.and_then(|a| a.get("type")).and_then(|v| v.as_str()).unwrap_or("bar");
                         let id = args.and_then(|a| a.get("id")).and_then(|v| v.as_str()).unwrap_or("c1");
                         Ok(json!({ "content": [{ "type": "text", "text": create_chart(t, id) }] }))
                     },
                     "daisyui_create_table" => {
                         Ok(json!({ "content": [{ "type": "text", "text": create_complex_table(&[]) }] }))
                     },
                     "daisyui_generate_theme" => {
                        let name = args.and_then(|a| a.get("name")).and_then(|v| v.as_str()).unwrap_or("mytheme");
                        let p = args.and_then(|a| a.get("primary")).and_then(|v| v.as_str()).unwrap_or("#000");
                        let b = args.and_then(|a| a.get("base")).and_then(|v| v.as_str()).unwrap_or("#fff");
                        Ok(json!({ "content": [{ "type": "text", "text": generate_theme(name, p, "", "", b) }] }))
                     },
                     "daisyui_scaffold_form" => {
                         let t = args.and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("Form");
                         Ok(json!({ "content": [{ "type": "text", "text": scaffold_form(t, &[]) }] }))
                     },
                     "daisyui_get_script" => {
                         let c = args.and_then(|a| a.get("component")).and_then(|v| v.as_str()).unwrap_or("");
                         Ok(json!({ "content": [{ "type": "text", "text": get_script(c) }] }))
                     },

                    _ => Err(JsonRpcError { code: -32601, message: format!("Unknown tool: {}", name), data: None })
                }
            } else {
                 Err(JsonRpcError { code: -32602, message: "Missing params".to_string(), data: None })
            }
        },
        _ => Err(JsonRpcError { code: -32601, message: "Method not found".to_string(), data: None })
    };

    match result {
        Ok(val) => JsonRpcResponse { jsonrpc: "2.0".to_string(), result: Some(val), error: None, id },
        Err(err) => JsonRpcResponse { jsonrpc: "2.0".to_string(), result: None, error: Some(err), id }
    }
}
