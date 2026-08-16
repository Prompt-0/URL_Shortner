## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-17 - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (, , ) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## 2026-08-16 - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## 2026-07-31 - Make form constraints explicitly visible and accessible
**Learning:** Found that custom validation patterns (like `pattern` and `minlength`) in the UI were missing visible helper text, which means users wouldn't know the constraints until they failed validation. Relying only on `title` or failing on submit causes frustration.
**Action:** When adding strict regex or pattern validations to UI inputs, accompany them with visible helper text and link it using `aria-describedby` so the constraints are announced upfront to screen readers, preventing validation errors before they occur.

## 2026-08-16 - Accessible Truncated Text
**Learning:** Using `text-overflow: ellipsis` without additional accessibility features causes screen reader users and keyboard users to lose important context, as the hidden text is inaccessible. Adding an anchor tag can introduce color contrast issues in dark themes.
**Action:** When using `text-overflow: ellipsis` in UI templates, ensure the truncated text is accessible. Providing a `title` attribute and `tabindex="0"` (with `:focus-visible` styling) on the container is preferred over adding a new anchor tag to avoid styling regressions.
