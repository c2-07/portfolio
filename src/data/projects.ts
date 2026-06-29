type Project = {
  name: string;
  description: string;
  tags: string[];
  link: string;
};

export const projects: Project[] = [
  {
    name: "transferThing",
    description: "CLI tool to share files over a local network.",
    tags: ["c", "python"],
    link: "https://github.com/c2-07/transferThing",
  },
  {
    name: "postOffice",
    description:
      "temporary file sharing service with direct uploads and downloads.",
    tags: ["fastapi", "sqlmodel", "sqlite"],
    link: "https://github.com/c2-07/postOffice",
  },
  {
    name: "portfolio",
    description: "personal portfolio website built with Astro.",
    tags: ["astro", "tailwindcss", "typescript"],
    link: "https://github.com/c2-07/portfolio",
  },
  {
    name: "wssThing",
    description: "minimal WebSocket library in C with TLS support.",
    tags: ["c", "networking", "websocket", "openssl"],
    link: "https://github.com/c2-07/wssThing",
  },
  {
    name: "subdog",
    description: "high-concurrency subdomain brute-forcer using async DNS.",
    tags: ["python", "asyncio", "dns", "networking"],
    link: "https://github.com/c2-07/subdog",
  },
  {
    name: "playlistThing",
    description:
      "cross-platform playlist transfer service with support for YouTube and Spotify.",
    tags: ["python", "fastapi", "spotify", "youtube"],
    link: "https://github.com/c2-07/playlistThing",
  },
  {
    name: "interpreter",
    description:
      "C++ implementation of the interpreter from Crafting Interpreters",
    tags: ["c++", "compiler", "interpreter"],
    link: "https://github.com/c2-07/interpreter",
  },
];
