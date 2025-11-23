# Insights from Building FrameFit with AI Assistance

Total Development Time: ~2-3 hours of active work

What we built:

    * Complete Tauri + React + TypeScript app from scratch
    * Rust backend with macOS window management (Core Foundation, Core Graphics,
      Cocoa)
    * Full UI with glass-morphism design
    * Preset management system with persistent storage
    * Import/export functionality
    * Window selection and resizing logic
    * Center window feature
    * Comprehensive README with documentation

Challenges encountered:

    1. **Initial compilation errors** (~30 min) - Fixed 22 Rust errors from
       outdated APIs
    2. **Icon corruption** (~10 min) - Had to work around invalid PNG files
    3. **AppleScript issues** (~20 min) - Refined window targeting and error
       handling
    4. **Permission setup** (~15 min) - Configured Tauri capabilities for
       store/dialog/fs
    5. **Preset storage** (~10 min) - Debugged store permissions

What made it fast:

    * Clear requirements and iterative feedback
    * Modern tooling (Tauri 2.0, Vite, TypeScript)
    * Reusable patterns for window management
    * Quick iteration cycles

Complexity breakdown:

    * Backend (Rust/macOS APIs): 40% of effort
    * Frontend (React/UI): 30% of effort
    * Bug fixes/permissions: 20% of effort
    * Documentation: 10% of effort

For a solo developer without AI assistance, this would typically take 2-3 days
to build from scratch. The biggest time-saver was handling the Rust/macOS
integration and debugging compilation errors quickly.