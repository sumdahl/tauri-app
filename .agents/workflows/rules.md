---
description: Pre-push quality checklist — verifies all project rules are met before code is pushed to GitHub.
---

# Pre-Push Rules Checklist

Run through every step below before pushing code.

// turbo-all

## 1. Rust Compilation Check
Ensure the project compiles without errors.
```bash
cd src-tauri && cargo check
```

## 2. Clippy Lint Check
Run Clippy to catch common mistakes and enforce idiomatic Rust.
```bash
cd src-tauri && cargo clippy -- -D warnings
```

## 3. No `unwrap()` in Production Code
Scan for any `unwrap()` or `expect()` calls in production source files (excluding tests and binaries used only for dev).
```bash
cd src-tauri && grep -rn 'unwrap()' src/lib.rs src/domain/ src/application/ src/infrastructure/ src/presentation/ --include='*.rs' || echo "✅ No unwrap() found"
```
```bash
cd src-tauri && grep -rn 'expect(' src/lib.rs src/domain/ src/application/ src/infrastructure/ src/presentation/ --include='*.rs' || echo "✅ No expect() found"
```
If any are found, refactor them to use `Result` with `thiserror`.

## 4. Tailwind CSS Builds Successfully
```bash
cd src-tauri && npm run build
```

## 5. No CDN References
Ensure no external CDN links exist in HTML files.
```bash
cd src-tauri && grep -rn 'cdn\|unpkg\|jsdelivr\|cloudflare' src/presentation/ --include='*.html' || echo "✅ No CDN references found"
```

## 6. Offline Assets Present
Verify that HTMX and Tailwind CSS are bundled locally.
```bash
ls src-tauri/src/presentation/assets/js/htmx.min.js && echo "✅ HTMX bundled" || echo "❌ HTMX missing!"
ls src-tauri/src/presentation/assets/css/style.css && echo "✅ Tailwind CSS built" || echo "❌ Tailwind CSS missing — run: npm run build"
```

## 7. DDD Structure Intact
Verify the Domain-Driven Design directories exist.
```bash
ls -d src-tauri/src/domain src-tauri/src/application src-tauri/src/infrastructure src-tauri/src/presentation && echo "✅ DDD structure intact" || echo "❌ Missing DDD directories!"
```

## 8. Ready to Push
If all checks above pass, the code is ready:
```bash
echo "🚀 All checks passed. Ready to push!"
```
