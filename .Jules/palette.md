## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-17 - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (, , ) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## $(date +%Y-%m-%d) - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## 2026-07-25 - Explicit Form Validation Helper Text
**Learning:** When using strict regex/pattern validations (like only letters, numbers, hyphens), relying solely on the `title` attribute or generic errors is insufficient for screen readers and user visibility. Linking explicit, visible helper text using `aria-describedby` prevents errors proactively.
**Action:** Always accompany complex `pattern` validations with visible helper text and link it properly via ARIA attributes.
