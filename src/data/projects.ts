type Project = {
  name: string;
  description: string;
  tags: string[];
  link: string;
};

export const projects: Project[] = [
  {
    name: "wifi-room-mapper",
    description:
      "indoor localization, Wi-Fi fingerprinting, and spatial surveying Flutter application with boundary-aware prediction engine.",
    tags: ["flutter", "dart", "indoor-localization", "networking"],
    link: "https://github.com/c2-07/wifi-room-mapper",
  },
  {
    name: "transferThing",
    description:
      "fast LAN file and directory transfer CLI with automatic peer discovery over UDP broadcast.",
    tags: ["go", "networking", "cli", "udp"],
    link: "https://github.com/c2-07/transferThing",
  },
  {
    name: "wssThing",
    description:
      "minimal RFC 6455 WebSocket library in C with TLS encryption and custom frame parsing.",
    tags: ["c", "networking", "websocket", "openssl"],
    link: "https://github.com/c2-07/wssThing",
  },
  {
    name: "interpreter",
    description:
      "tree-walk programming language interpreter written in C++ with custom lexer, recursive descent parser, and runtime.",
    tags: ["c++", "compiler", "interpreter", "parsing"],
    link: "https://github.com/c2-07/interpreter",
  },
  {
    name: "zahard",
    description:
      "full-stack AI assistant platform built with Astro, Vercel AI SDK, and PostgreSQL, accompanied by a native Rust CLI.",
    tags: ["astro", "rust", "typescript", "ai", "postgres"],
    link: "https://github.com/c2-07/zahard",
  },
  {
    name: "postOffice",
    description:
      "full-stack temporary file sharing service featuring a React frontend, FastAPI backend, and Supabase authentication.",
    tags: ["fastapi", "react", "python", "sqlite", "supabase"],
    link: "https://github.com/c2-07/postOffice",
  },
  {
    name: "fortinet-auto-login",
    description:
      "zero-dependency cross-platform Go CLI automating login, logout, and keepalive for FortiGate captive network portals.",
    tags: ["go", "networking", "automation", "cli"],
    link: "https://github.com/c2-07/fortinet-auto-login",
  },
  {
    name: "subdog",
    description:
      "high-concurrency subdomain brute-forcer and DNS resolver leveraging Python asyncio.",
    tags: ["python", "asyncio", "dns", "networking"],
    link: "https://github.com/c2-07/subdog",
  },
  {
    name: "nanofetch",
    description:
      "ultra-minimal, zero-dependency system information fetch tool written in C with precomputed ANSI escapes.",
    tags: ["c", "cli", "linux"],
    link: "https://github.com/c2-07/nanofetch",
  },
  {
    name: "bocchi",
    description:
      "minimal aesthetic lofi player web app featuring ambient sound controls, live clock, and task tracker.",
    tags: ["astro", "tailwindcss", "typescript", "audio"],
    link: "https://github.com/c2-07/bocchi",
  },
  {
    name: "portfolio",
    description:
      "terminal-inspired personal portfolio and logbook built with Astro, Tailwind CSS, and TypeScript.",
    tags: ["astro", "tailwindcss", "typescript"],
    link: "https://github.com/c2-07/portfolio",
  },
];
