# Monolith

**Monolith** is an under development high-performance, open-source BIM platform engineered in Rust to obliterate the single-core bottlenecks of 90s-era legacy software. Powered by a multi-threaded, GPU-accelerated database engine, it consolidates the fragmented workflow of Revit, Navisworks, and Excel into a single, unified environment for real-time 3D design, 4D scheduling, and 5D cost estimation.

**End Goal**: Modernize the BIM Software used in the industry

# Run

`cargo run -p mn_app`

# Progress

The current objective is to build an MVP with the ff features

- Functional Window with 3D Viewport ✅

- Grid System

- Basic Architecture BIM

- Basic Asset creation

- Functioning sqlite/duckdb integration

- PDF Creation

# Planned Features

- **Database-Native Core**
  
  - Built on SQLite & DuckDB to ensure zero data loss and enable instant, complex analytics without proprietary file corruption.

- **High-Performance Engine**
  
  - Powered by the Bevy ECS, handling hundreds of thousands of elements at 60 FPS with automatic LOD, mesh decimation, and occlusion culling.
  
  - True multi-threading and GPU acceleration eliminate the single-core CPU bottlenecks that slow down legacy BIM tools (Revit).

- **Blender-Inspired UX**
  
  - A modern, dark mode, fast, tiling window manager designed to maximize workflow efficiency.

- **Real-Time Clash Detection**
  
  - Native, instant clash detection identifies spatial conflicts as you model, removing the need for external tools like Navisworks.

- **Git-Style Collaboration**
  
  - Uses optimistic concurrency. Only delta changes are synchronized (not the entire file). This allows real-time collaboration where other user actions can be seen.
  
  - Locks distinct attributes (not entire objects) and manages merges through a dedicated resolution UI.
  
  - Work Space. A space user can define to signal to other users that they are working in this space (does not lock the space)

- **4D Timeline**
  
  - Replaces rigid "Phase Dropdowns" with a scrubbable* video-editor* like timeline to visualize construction sequences (Existing → Demo → New) in real-time.

- **Live 5D Costing**
  
  - Generates instant, always-up-to-date Bill of Quantities (BOQ) and cost estimates that react immediately to geometry changes.

- **Unified Visibility Stack**
  
  - Replaces scattered graphic menus with a clean, CSS-style rule system for controlling element visibility, eliminating the "hidden element hunt."
  
  - Graphic Override/Modification are all listed in a stack
    
    - e.g. a hidden element is a single stack, so is a category override or element override
  
  - Each views has an Active Override and Template Override. 

- **Offline AI Command Parser** 
  
  - A local AI assistant that executes complex natural language commands without sending data to the cloud.