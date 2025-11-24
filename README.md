<div align="center">
  <img src="app-icon.png" alt="FrameFit Icon" width="128" height="128">
                
  # FrameFit
                
  **A beautiful macOS app to quickly resize any window to preset or custom
  dimensions**
                
              
            
          
        
      
    
  
  [![macOS](https://img.shields.io/badge/macOS-10.15+-blue.svg)](https://www.apple.com/macos/)
  [![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB.svg)](https://tauri.app/)
  [![TypeScript](https://img.shields.io/badge/TypeScript-5.0-3178C6.svg)](https://www.typescriptlang.org/)
  [![React](https://img.shields.io/badge/React-19.1-61DAFB.svg)](https://reactjs.org/)
                
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
</div>



## ✨ Features

- 🎯 **Quick Presets** - Instantly resize windows to common device dimensions
  (iPhone, iPad, Desktop)
- 🎨 **Custom Sizes** - Set any width and height for precise window dimensions
- 📍 **Auto-Center** - Optionally center windows on screen after resizing
- 💾 **Custom Presets** - Save your frequently used sizes for quick access
- 📤 **Import/Export** - Share preset configurations with your team
- 🪟 **Window Selection** - Choose specific windows or resize the topmost window
- 🎭 **Beautiful UI** - Modern glass-morphism design with smooth animations
- ⚡ **Native Performance** - Built with Tauri for minimal resource usage

## 📸 Screenshots

<div align="center">
  <img src="screenshots/main-v0.2.0.png" alt="FrameFit Main Interface" width="600">
  <p><em>Clean, intuitive interface with preset sizes</em></p>
</div>

## 🚀 Getting Started

### Prerequisites

- macOS 10.15 (Catalina) or later
- **Accessibility Permissions** - Required to resize windows

### Installation

#### Option 1: Download Release (Coming Soon)
Download the latest `.dmg` from the
[Releases](https://github.com/estruyf/FrameFit/releases) page.

#### Option 2: Build from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/estruyf/FrameFit.git
   cd framefit-mac
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

### First Run Setup

1. Launch FrameFit
2. Grant **Accessibility Permissions** when prompted:
   - Open **System Settings** → **Privacy & Security** → **Accessibility**
   - Enable FrameFit in the list
3. Restart the app if needed

## 📖 Usage

### Quick Resize
1. Click a preset size (iPhone SE, iPad, HD, etc.)
2. Click **"Resize Top Window"** to resize the frontmost window
3. Or click **"Select Window"** to choose a specific window

### Custom Sizes
1. Enter your desired **Width** and **Height**
2. Toggle **"Center window after resize"** if desired
3. Click **"Resize Top Window"** or select a specific window

### Managing Presets

#### Add Custom Preset
1. Set your desired width and height
2. Click the **"+"** button
3. Enter a name for your preset
4. Click **Save**

#### Delete Custom Preset
- Click the **×** button on any custom preset
- *Note: Default presets cannot be deleted*

#### Export/Import Presets
1. Click the **⋮** menu button
2. Choose **"Export Presets"** to save your custom presets
3. Choose **"Import Presets"** to load presets from a file
4. Choose **"Reset to Defaults"** to remove all custom presets

## 🎯 Default Presets

| Preset | Width | Height | Use Case |
|--------|-------|--------|----------|
| iPhone SE | 375 | 667 | Mobile testing (small) |
| iPhone 14 | 390 | 844 | Mobile testing (standard) |
| iPad | 768 | 1024 | Tablet testing |
| HD | 1280 | 720 | Desktop (720p) |
| FHD | 1920 | 1080 | Desktop (1080p) |

## 🛠️ Technology Stack

- **[Tauri 2.0](https://tauri.app/)** - Lightweight desktop framework
- **[React 18](https://reactjs.org/)** - UI framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe development
- **[Vite](https://vitejs.dev/)** - Fast build tool
- **Rust** - Backend for native macOS integration
- **AppleScript** - Window manipulation on macOS

## 🏗️ Architecture

```
framefit-mac/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── App.css            # Styles with glass-morphism
│   └── main.tsx           # React entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Tauri commands
│   │   └── window_manager.rs  # macOS window operations
│   ├── Cargo.toml         # Rust dependencies
│   └── capabilities/      # Tauri permissions
└── public/                # Static assets
```

## 🔒 Permissions

FrameFit requires the following permissions:

- **Accessibility** - To read window information and resize windows
- **File System** - To save/load preset configurations
- **Dialog** - To show file picker for import/export

All permissions are used solely for the app's core functionality and no data is
collected or transmitted.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 Development

### Project Structure
- Frontend: React + TypeScript + Vite
- Backend: Rust + Tauri
- Window Management: Core Foundation + Core Graphics + AppleScript

### Key Files
- `src/App.tsx` - Main UI component
- `src-tauri/src/window_manager.rs` - Window operations
- `src-tauri/src/lib.rs` - Tauri command handlers

### Commands
```bash
npm run dev          # Run frontend dev server
npm run tauri dev    # Run full app in dev mode
npm run build        # Build frontend
npm run tauri build  # Build production app
```

## 🐛 Known Issues

- Some apps may not support programmatic resizing
- Window centering uses primary display dimensions
- Requires Accessibility permissions to function

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## 🙏 Acknowledgments

- Inspired by [ResizeMe](https://burkeholland.github.io/ResizeMe/) by
  [Burke Holland](https://github.com/burkeholland)
- Built with [Tauri](https://tauri.app/)
- Created with [CommandCode.ai](https://commandcode.ai) - AI-powered coding
  assistant
- Icons and design inspired by modern macOS apps
- Community feedback and contributions

## 📧 Contact

- **Issues**: [GitHub Issues](https://github.com/estruyf/FrameFit/issues)
- **Discussions**:
  [GitHub Discussions](https://github.com/estruyf/FrameFit/discussions)



<div align="center">
  Made with ❤️ for macOS developers and designers
                
  **[⬆ back to top](#framefit)**
</div>

<div align="center">
   <a
   href="https://visitorbadge.io/status?path=https%3A%2F%2Fgithub.com%2Festruyf%2FFrameFit"><img
   src="https://api.visitorbadge.io/api/visitors?path=https%3A%2F%2Fgithub.com%2Festruyf%2FFrameFit&countColor=%23263759"
   /></a>
</div>