# CharlotteOS License Clarification: Closed Binary Driver Linking for Private Use

This note documents the interpretation of the **GNU General Public License, version 3 (or any later version)**, as agreed upon by all copyright
holders of this project with respect to linking closed-source binary drivers.

---

## Statement of Interpretation

We interpret the GNU GPLv3 to require source code disclosure **only when a covered work is _conveyed_ to a third party**, as defined in Section 0 of the license.  
**Private use** — including modifications, combinations, and compilations — is not regulated by the GPL unless and until the resulting work is conveyed.

Accordingly:

- Users **may link this kernel with closed-source binary drivers**, including static libraries, **for personal, internal, or evaluation use**  
  without being required to disclose the source code of the proprietary driver.

- This applies whether linking is performed manually or by automated build systems, package managers, or similar tools, **provided that the resulting binaries are not conveyed to others**.

- If the combined work is ever conveyed (e.g., shared, sold, or distributed), **all GPLv3 obligations** — including source code disclosure for the entire combined work — **apply in full**, as specified by the license.

---

## Scope and Purpose

This is **not an exception** or a modification to the terms of the GPLv3.  
It is a statement of how we, the copyright holders,
**interpret the license’s scope with regard to private, non-conveyed use**.

This clarification is provided to reduce uncertainty for users and developers working with closed binary drivers in non-distributed contexts.

---

## Contributor Acknowledgment

By contributing to this project, **you affirm that you agree with this interpretation of the GPLv3 license** as stated above.  
All contributors are assumed to consent to this interpretation as a condition of their participation.

If you **do not agree** with this interpretation, **please do not contribute** to this project.

This ensures a consistent and transparent legal foundation for the project, its users, and its community.

---

## Effective Scope

This interpretation applies to **all components of the kernel** covered under this project’s **GPLv3-or-later license**,  
and represents the **unanimous position of all contributors** as to how the license should be understood in this context.
