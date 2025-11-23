---
title: "Building FrameFit in 3 Hours: How AI Accelerated My macOS App Development"
slug: building-framefit-macos-app-with-ai-3-hours
description: "I needed consistent screenshots for documentation, so I built a macOS window resizer in just 3 hours using Tauri and AI. Here's how AI-powered development changed everything."
date: 2025-11-23T10:00:00.000Z
lastmod: 2025-11-23T10:00:00.000Z
draft: false
tags:
  - macOS
  - Tauri
  - AI
  - Development
  - Productivity
categories:
  - Development
type: post
---

I have a problem that many technical writers and product managers will recognize: inconsistent screenshots in documentation. When you're creating product images and documentation, having windows at different sizes makes everything look unprofessional. One screenshot shows a browser at 1200px wide, the next at 1400px, and suddenly your documentation looks messy.

I needed a tool to quickly resize windows to exact dimensions so every screenshot would be consistent. I found [Burke Holland's ResizeMe](https://burkeholland.github.io/ResizeMe/), which was close to what I needed, but I wanted something tailored to my workflow with a few extra features.

So I decided to build **FrameFit**—and with the help of AI, I had a fully functional macOS app in just 3 hours.

## The Problem: Consistent Screenshots

When you're documenting software, consistency matters. Your screenshots should:

- Use the same window dimensions across all images
- Look professional and polished
- Be easy to reproduce when you need to update them
- Work for different device sizes (mobile, tablet, desktop)

Manually resizing windows every time is tedious and error-prone. I'd resize a window, take a screenshot, then later realize I used slightly different dimensions. The result? Documentation that looked inconsistent and unprofessional.

## The Solution: Build It Myself

I wanted a simple app that could:

- Resize any window to preset dimensions (iPhone, iPad, desktop sizes)
- Support custom dimensions for specific use cases
- Center windows after resizing for perfect screenshots
- Save my frequently used sizes as custom presets
- Be fast and lightweight

Instead of spending days building this, I decided to use [CommandCode.ai](https://commandcode.ai) to accelerate the development process.

## The AI-Powered Development Experience

### Hour 1: Getting Started

I started by explaining what I wanted to build. Within minutes, we had:

- A fresh Tauri 2.0 project with TypeScript template
- Basic project structure set up
- Dependencies configured

The AI suggested using Tauri because it's lightweight (unlike Electron) and perfect for system-level operations like window management. I agreed, and we were off to the races.

**Traditional approach**: Would have taken me 2-3 hours just researching which framework to use, reading documentation, and setting up the project correctly.

**With AI**: 15 minutes.

### Hour 2: Building the Core Features

This is where AI really shined. I needed to:

1. Interface with macOS APIs (Core Foundation, Core Graphics, Cocoa)
2. Build a Rust backend for window management
3. Create Tauri commands to expose functionality to the frontend
4. Design a React UI with preset buttons and custom inputs

I don't write Rust regularly, and I've never worked with macOS window management APIs. Normally, this would mean:

- Hours reading documentation
- Trial and error with unsafe Rust code
- Debugging cryptic macOS API errors
- Learning AppleScript for window manipulation

**With AI**: We built the entire backend in about 45 minutes. The AI knew:

- Which macOS APIs to use
- How to safely interface with C APIs from Rust
- How to handle permissions and edge cases
- How to use AppleScript for window resizing

When we hit compilation errors (22 of them from outdated API usage), the AI quickly identified and fixed each one. What would have been hours of Stack Overflow searching was resolved in minutes.

### Hour 3: Polish and Features

With the core functionality working, we added:

- **Beautiful UI** - Glass-morphism design with gradient backgrounds
- **Preset management** - Save, export, and import custom presets
- **Center window feature** - Perfect for screenshots
- **Window selection** - Choose specific windows or resize the topmost one
- **Permissions handling** - Detect and warn about missing Accessibility permissions

The AI helped design the UI, implement the store plugin for persistence, and add all the polish that makes an app feel professional.

## The Results

**Total development time**: ~3 hours of active work

**What I got**:
- ✅ Fully functional macOS app
- ✅ Beautiful, modern UI
- ✅ Preset management with import/export
- ✅ Window centering for perfect screenshots
- ✅ Native performance (thanks to Tauri)
- ✅ Comprehensive documentation

**What it would have taken without AI**: 2-3 days minimum

Here's the breakdown:

| Task | Traditional Time | With AI | Time Saved |
|------|-----------------|---------|------------|
| Project setup & research | 2-3 hours | 15 min | 2+ hours |
| Rust backend + macOS APIs | 8-10 hours | 45 min | 8+ hours |
| UI implementation | 4-5 hours | 1 hour | 3+ hours |
| Bug fixes & debugging | 3-4 hours | 30 min | 3+ hours |
| Documentation | 2 hours | 30 min | 1.5 hours |
| **Total** | **19-24 hours** | **3 hours** | **17+ hours** |

## How AI Changed the Game

### 1. **No More Context Switching**

Normally, building this would require:
- Reading Tauri documentation
- Learning Rust window management
- Researching macOS APIs
- Looking up AppleScript syntax
- Finding React component examples

With AI, I stayed in flow. I described what I needed, and the AI provided working code with explanations.

### 2. **Instant Expertise**

I'm not a Rust expert. I don't know macOS APIs. But with AI, I didn't need to be. The AI brought expertise in:
- Rust unsafe code patterns
- macOS Core Foundation APIs
- Tauri plugin system
- React best practices
- TypeScript type safety

### 3. **Rapid Iteration**

When something didn't work (like windows not showing up or presets not saving), we fixed it immediately. No waiting for Stack Overflow answers or digging through documentation.

Example: When the app tried to resize itself instead of other windows, the AI immediately suggested filtering out the app from the window list. Problem solved in 2 minutes.

### 4. **Learning While Building**

The AI explained *why* certain approaches worked. I learned about:
- How macOS window management works
- Tauri's permission system
- Rust's ownership model in practice
- AppleScript for system automation

I didn't just get a working app—I understood how it worked.

## The Real-World Impact

Now when I create documentation:

1. Open FrameFit
2. Click "iPhone 14" preset (390×844)
3. Take screenshot
4. Every screenshot is exactly 390×844

My documentation looks professional and consistent. No more manually resizing windows or fixing inconsistent screenshots.

**Bonus**: I can share my custom presets with my team, so everyone's screenshots match.

## What This Means for Development

This experience changed how I think about building tools:

### Before AI:
- "I need this tool, but it'll take days to build"
- "I don't know Rust/macOS APIs, so I can't build this"
- "I'll just deal with the manual process"

### With AI:
- "I need this tool, let's build it this afternoon"
- "I don't need to be an expert—AI can help"
- "I can build exactly what I need"

The barrier to building custom tools has dropped dramatically. If you have a problem and a few hours, you can build a solution.

## Lessons Learned

### 1. **Start Fresh When Possible**

I initially tried fixing an existing project but hit too many issues. Starting fresh with a new Tauri template was faster and cleaner.

### 2. **AI Excels at Unfamiliar Territory**

The biggest time savings came from areas I didn't know well (Rust, macOS APIs). AI filled those knowledge gaps instantly.

### 3. **Focus on the Problem, Not the Implementation**

I spent my time thinking about features and user experience, not fighting with APIs and syntax. That's how it should be.

### 4. **MVPs Are Now Hours, Not Days**

Getting a working prototype used to take days. Now it takes hours. This changes what's worth building.

### 5. **You Still Need to Understand**

AI wrote the code, but I made the decisions. Understanding what you're building and why still matters.

## What's Next

FrameFit works great for my needs, but there are features I'd like to add:

- **Global keyboard shortcuts** - Resize without opening the app
- **Menu bar icon** - Quick access from anywhere
- **Percentage-based sizing** - Resize to 50%, 75% of screen
- **Multi-monitor support** - Choose which display to use
- **Window layouts** - Save entire workspace arrangements

With AI, adding these features will be quick. Each one is probably 30-60 minutes of work.

## Try It Yourself

FrameFit is open source and available on [GitHub](https://github.com/yourusername/framefit-mac). If you need consistent screenshots for documentation, give it a try.

More importantly: if you have a problem that needs a custom tool, don't let lack of expertise stop you. With AI-powered development, you can build it yourself in hours, not days.

---

**Links:**
- [FrameFit on GitHub](https://github.com/yourusername/framefit-mac)
- [CommandCode.ai](https://commandcode.ai) - The AI coding assistant I used
- [Tauri](https://tauri.app/) - The framework that made this possible
- [Original ResizeMe by Burke Holland](https://burkeholland.github.io/ResizeMe/) - The inspiration

**The bottom line**: I went from "I need this tool" to "I have this tool" in one afternoon. That's the power of AI-accelerated development.

Have you built anything with AI assistance? What tools have you been wanting to build but haven't had time? Let me know in the comments!
