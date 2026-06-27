type Project = {
  name: string;
  description: string;
  tags: string[];
  link: string;
};

export const projects: Project[] = [
  {
    name: "transferThing",
    description: "cli-tool to share files over local network",
    tags: ["c", "python"],
    link: "https://github.com/c2-07/transferthing",
  },
  {
    name: "postOffice",
    description:
      "temporary file sharing service with direct uploads and downloads.",
    tags: ["fastapi", "sqlmodel", "sqlite"],
    link: "https://github.com/c2-07/postOffice",
  },
  {
    name: "wssThing",
    description: "minimal websocket library in c with tls support.",
    tags: ["c", "networking"],
    link: "https://github.com/c2-07/wssThing",
  },
  {
    name: "subdog",
    description: "high-concurrency subdomain brute-forcer using async dns.",
    tags: ["python", "networking"],
    link: "https://github.com/c2-07/subdog",
  },
];
