import { assertEquals } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { calculateId } from "./part1.ts";

Deno.test("calculates ids correctly", () => {
  const converted = ["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
    .map(calculateId);

  const expected = [357, 567, 119, 820];

  assertEquals(converted, expected);
});
