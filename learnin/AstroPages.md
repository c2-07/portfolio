# Pages

> If we have:

```text
src/
└── pages/
    ├── index.astro
    ├── about.astro
    └── blog/
        └── first-post.astro
```

Astor automatically creates these routes:

```text
src/pages/index.astro              /

src/pages/about.astro              /about

src/pages/blog/first-post.astro    /blog/first-post
```

---

A page file can:

1. Handle routing : The file location determines the URL.
   `src/pages/contact.astro  -> /contact`

---

Let's say if want to create a link which navigates to other page. Normally we
would create a relative link, but in Astro we'll use `URL PATH REALTIVE to our
domain`
