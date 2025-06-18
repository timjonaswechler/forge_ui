# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Forge UI is a Rust-based UI component library built on Bevy 0.16 game engine. The project converts Radix UI TypeScript components into Bevy-compatible Rust components for interactive story-based games. The architecture follows an Entity-Component-System (ECS) pattern with state management, theming, and hot-reload capabilities.

## Architecture

### Core Plugin Structure
- **ForgeUiPlugin**: Main plugin that manages UI state transitions through `UiState` enum
- **State Management**: Asset loading → Theme loading → Ready → Hot reload cycle
- **Component System**: Each UI component is a separate plugin with builder pattern, components, events, and systems

### Key Directories
- `src/components/`: Individual UI components (buttons, dialogs, etc.)
- `src/showcase/`: Demo implementations showing component usage
- `src/theme/`: Theming system with runtime and data modules
- `src/assets/`: Font and icon asset management
- `assets/`: SVG and PNG icon resources

### Component Structure
Each component follows a standardized pattern:
- `builder.rs`: Builder pattern for component creation
- `components.rs`: Bevy component structs and markers
- `events.rs`: Component-specific events
- `systems.rs`: ECS systems handling component behavior
- `style.rs`: Styling and theme integration
- `plugin.rs`: Bevy plugin registration

## Development Commands

### Build and Check
```bash
cargo check          # Quick compilation check
cargo build          # Full build
cargo build --features showcase  # Build with showcase components
```

### Testing
The project uses cargo's built-in test framework. Run tests with:
```bash
cargo test
```

### Hot Reload Development
The theme system supports hot reload. Press 'S' key during runtime to save current theme state.

## AI Agent Workflow

This project has a sophisticated AI agent workflow defined in `AGENTS.md`:

1. **Task Selection**: Check `MASTER_TASK_LIST.md` for next uncompleted component
2. **Dependency Analysis**: Ensure all component dependencies are implemented first
3. **Implementation Strategy**: 
   - New components: Full implementation from scratch
   - Existing components: Extension/enhancement of current functionality
4. **Validation**: Ensure `cargo check` passes after changes

### Error Checking and Documentation
**CRITICAL**: Every time you implement code, you MUST:
1. **Self-check for errors**: After writing any code, always run `cargo check` to identify compilation errors, warnings, and type issues
2. **Fix errors immediately**: Address all compilation errors, warnings, and clippy suggestions before proceeding
3. **Consult Bevy documentation**: For better understanding of Bevy APIs, always reference the official docs.rs documentation at https://docs.rs/bevy/0.16.0/bevy/
4. **Verify component patterns**: Ensure all ECS patterns follow Bevy best practices as documented in the official Bevy book

### Critical Files for AI Agents
- `AGENTS.md`: Complete workflow algorithm for component implementation
- `MASTER_TASK_LIST.md`: Single source of truth for component backlog
- `src/components/TEMPLATE.md`: Standardized component structure template

## Component Implementation Guidelines

### For New Components
1. Create module directory under `src/components/`
2. Implement all required files following TEMPLATE.md structure
3. Register plugin in `src/plugin.rs`
4. Add showcase implementation in `src/showcase/components/`
5. Update `src/components/mod.rs` with new module

### For Component Extensions
1. Analyze existing implementation vs Radix UI reference
2. Identify missing props, variants, or behaviors  
3. Extend existing structs and systems rather than replacing
4. Update showcase to demonstrate new functionality

## Bevy-Specific Patterns

### ECS Integration
- UI components are Bevy entities with component markers
- Use `Commands` for entity spawning and manipulation  
- Systems handle component logic and state changes
- Events communicate between components

### Asset Management
- Icons loaded via `IconAssets` resource using bevy_asset_loader
- Fonts managed through `FontAssets` 
- Theme data loaded from RON files with hot reload support

### State Management
- Global UI state managed through `UiState` resource
- Component-specific state via marker components
- Events for inter-component communication

## Theme System

The project uses a comprehensive theming system:
- **Runtime**: Dynamic theme application during gameplay
- **Data**: Static theme definitions loaded from assets
- **Settings**: User preferences for appearance, scaling, etc.

Theme integration happens automatically through the styling systems in each component.

## Radix UI Conversion Notes

Components are converted from Radix UI maintaining:
- **Props compatibility**: Size, variant, color, highContrast properties
- **Behavior consistency**: Interaction patterns and state management
- **Accessibility**: Screen reader support through appropriate component markers
- **Styling**: CSS classes converted to Bevy styling systems

Reference the original Radix UI documentation linked in `MASTER_TASK_LIST.md` for each component's expected behavior and API.