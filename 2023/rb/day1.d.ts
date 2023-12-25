import { StringToInteger } from './integers';

type Lines<
  T extends string,
  Acc extends string[] = []
> = T extends `${infer Head}\n${infer Tail}`
  ? Lines<Tail, [...Acc, Head]>
  : [...Acc, T];

type Chars<
  T extends string,
  Acc extends string[] = []
> = T extends `${infer C}${infer Rest}`
  ? Chars<Rest, [...Acc, C]>
  : T extends ''
  ? Acc[number]
  : [...Acc, T][number];

type First<T extends any[]> = T extends [infer First, ...infer Rest]
  ? First
  : never;

type Last<T extends any[]> = T extends [...infer Rest, infer Last]
  ? Last
  : never;

type Tail<T extends any[]> = T extends [infer First, ...infer Rest]
  ? Rest
  : never;

type Scan<
  T extends string,
  P extends string,
  Acc extends string[] = []
> = T extends `${infer H}${infer R}`
  ? H extends P
    ? Scan<R, P, [...Acc, H]>
    : Scan<R, P, Acc>
  : Acc;

type ScanLines<
  L extends string[],
  P extends string,
  Acc extends any[] = []
> = L extends [infer Next, ...infer Rest]
  ? ScanLines<Rest, P, [...Acc, Scan<Next, P>]>
  : Acc;

type Concat<T extends [string, string]> = `${First<T>}${Last<T>}`;

type MapIntegers<T extends string[][], Acc extends string[] = []> = T extends [
  infer H,
  ...infer Rest
]
  ? MapIntegers<Rest, [...Acc, Concat<[First<H>, Last<H>]>]>
  : Acc;

type ConvertStringsToIntegers<
  T extends string[],
  Acc extends number[] = []
> = T extends [infer H, ...infer Rest]
  ? ConvertStringsToIntegers<Rest, [...Acc, StringToInteger<H>]>
  : Acc;

type Length<T extends any[]> = T extends { length: infer L } ? L : never;

type BuildTuple<L extends number, T extends any[] = []> = T extends {
  length: L;
}
  ? T
  : BuildTuple<L, [...T, any]>;

type Add<A extends number, B extends number> = Length<
  [...BuildTuple<A>, ...BuildTuple<B>]
>;

type AddResults<T extends number[], Acc extends number = 0> = T extends [
  infer H,
  ...infer Rest
]
  ? AddResults<Rest, Add<Acc, H>>
  : Acc;

type NumberWords = [
  'zero',
  'one',
  'two',
  'three',
  'four',
  'five',
  'six',
  'seven',
  'eight',
  'nine'
];

type NumberWord = NumberWords[number];

type IndexOf<
  T extends any,
  Arr extends any[],
  Index extends number = 0
> = Arr extends [infer H, ...infer Rest]
  ? T extends H
    ? Index
    : IndexOf<T, Rest, Add<Index, 1>>
  : never;

type Part1 = AddResults<
  ConvertStringsToIntegers<
    MapIntegers<
      ScanLines<
        Lines<'1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet'>,
        Chars<'0123456789'>
      >
    >
  >
>;
