export const prerender = false;

import type { APIRoute } from "astro";
import { getNowPlaying } from "../../lib/spotify";

let cachedLastPlayed: any = null;

export const GET: APIRoute = async () => {
  try {
    const response = await getNowPlaying();

    if (!response || response.status === 204 || response.status > 400) {
      if (cachedLastPlayed) {
        return new Response(JSON.stringify({ ...cachedLastPlayed, isPlaying: false }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        });
      }
      return new Response(JSON.stringify({ isPlaying: false }), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });
    }

    const song = await response.json();

    if (song.item === null) {
      if (cachedLastPlayed) {
        return new Response(JSON.stringify({ ...cachedLastPlayed, isPlaying: false }), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        });
      }
      return new Response(JSON.stringify({ isPlaying: false }), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });
    }

    const isPlaying = song.is_playing;
    const title = song.item.name;
    const artist = song.item.artists
      .map((_artist: any) => _artist.name)
      .join(", ");
    const album = song.item.album.name;
    const albumImageUrl = song.item.album.images[0].url;
    const songUrl = song.item.external_urls.spotify;

    const payload = {
      album,
      albumImageUrl,
      artist,
      songUrl,
      title,
    };
    
    if (isPlaying) {
      cachedLastPlayed = payload;
    }

    return new Response(
      JSON.stringify({
        ...payload,
        isPlaying,
      }),
      {
        status: 200,
        headers: { "Content-Type": "application/json" },
      },
    );
  } catch (error) {
    console.error("Spotify API Error:", error);
    if (cachedLastPlayed) {
      return new Response(JSON.stringify({ ...cachedLastPlayed, isPlaying: false }), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });
    }
    return new Response(JSON.stringify({ isPlaying: false }), {
      status: 500,
      headers: { "Content-Type": "application/json" },
    });
  }
};
