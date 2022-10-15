import * as React from "react";
import { SVGProps } from "react";

const LinkExternal = (props: SVGProps<SVGSVGElement>) => (
  <svg viewBox="0 0 24 25" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path d="M15.5 2.907a.75.75 0 0 1 .75-.75h5.5a.75.75 0 0 1 .75.75v5.5a.75.75 0 0 1-1.5 0v-3.69l-6.22 6.22a.75.75 0 1 1-1.06-1.06l6.22-6.22h-3.69a.75.75 0 0 1-.75-.75Z" />
    <path d="M2.5 4.907c0-.967.784-1.75 1.75-1.75h8.5a.75.75 0 0 1 0 1.5h-8.5a.25.25 0 0 0-.25.25v15.5c0 .138.112.25.25.25h15.5a.25.25 0 0 0 .25-.25v-8.5a.75.75 0 0 1 1.5 0v8.5a1.75 1.75 0 0 1-1.75 1.75H4.25a1.75 1.75 0 0 1-1.75-1.75v-15.5Z" />
  </svg>
);

export default LinkExternal;
