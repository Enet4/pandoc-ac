Introduction to this Test
=========================

This is a text in which I will be adding multiple acronyms and expect it to work. Like (+.pacs). Even (+~pacs), and maybe even (+-*rbm).

How it works:

- `(+x)`: basic acronym instruction, translates to `\ac{x}`.
- `(+~x)`: full form of the acronym, translates to `\acf{x}`.
- `(+-x)`: always short form, translates to `\acs{x}`.
- `(+-x)`: always expand acronym, translates to `\acl{x}`.
- `(+*x)`, `(+.*x)`, `(+-*x)`, `(+~*x)`: plural form of the above, translate to `\acp{x}`, `\acsp{x}`, `\aclp{x}`, `\acfp{x}` respectively.
- `(+^x)`, `(+.^x)`, `(+-^x)`, `(+~^x)`: plural form, same thing as the previous point

The stuff in monospace formatting should not be touched. Not even things in equations: `a (+b)` $a (+b)$ or $$c (+d)$$.

- `(+cbir) is equal to \ac{cbir}`

   - (+cbir) is equal to \ac{cbir}

- `(+cbir) is equal to \ac{cbir}`

   - (+cbir) is equal to \ac{cbir}

- `(+~sdae) is equal to \acf{sdae}`

   - (+~sdae) is equal to \acf{sdae}

- `(+*api) is equal to \acp{api}`

   - (+*api) is equal to \acp{api}

- `(+-^api) is equal to \aclp{api}`

   - (+-^api) is equal to \aclp{api}

Acronym Inside headers: (+cbir)
-------------------------------

Should work even when surrounded by characters: !(+~xyz)!

> Should also work inside quotes: (+ooo)

Should work inside bullet points:

- (+dicom)
- (+nifti)
- (+hl7)

Should work inside enumerations:

1. (+-sdae): uses $L_1$-norm
2. (+-vae): uses KL divergence

Should also work surrounded by parentheses: ((+^api) (+rest))

Should also work inside tables:

| Acronym (like (+vga)) | Expanded (like (+-vga)) |
|-----------------------|-------------------------|
| (+-cga)               | (+-cga)                 |
| (+-ega)               | (+-ega)                 |
| (+-vga)               | (+-vga)                 |
| (+-svga)              | (+-svga)                |

  : List of video adapter formats, such as (+cga).
