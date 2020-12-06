import { path } from "./deps.ts";

export const readFile = (url: string): Promise<string> =>
  Deno.readTextFile(
    path.join(path.fromFileUrl(path.dirname(url)), "data.txt"),
  );
