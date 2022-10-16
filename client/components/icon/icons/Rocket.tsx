import * as React from "react";
import { SVGProps } from "react";

const Rocket = (props: SVGProps<SVGSVGElement>) => (
  <svg viewBox="0 0 24 25" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path
      fillRule="evenodd"
      clipRule="evenodd"
      d="M20.322 1.407a10.75 10.75 0 0 0-7.373 2.926l-1.304 1.23a23.743 23.743 0 0 0-1.542 1.594H5.066a1.75 1.75 0 0 0-1.5.85l-2.71 4.514a.75.75 0 0 0 .49 1.12l4.571.963c.039.049.082.096.129.14l1.995 1.873 1.872 1.995c.044.047.091.09.14.128l.963 4.572a.75.75 0 0 0 1.12.488l4.514-2.708a1.75 1.75 0 0 0 .85-1.5v-5.038a23.764 23.764 0 0 0 1.596-1.543l1.228-1.303a10.75 10.75 0 0 0 2.925-7.375V3.157a1.75 1.75 0 0 0-1.75-1.75h-1.177ZM16 15.77c-.333.248-.672.488-1.018.718L11.59 18.75l.678 3.223 3.612-2.168a.25.25 0 0 0 .121-.214V15.77Zm-10.092-2.7L8.17 9.674c.23-.345.47-.684.717-1.017H5.066a.25.25 0 0 0-.214.122L2.685 12.39l3.223.679Zm8.07-7.644a9.25 9.25 0 0 1 6.344-2.518h1.177a.25.25 0 0 1 .25.25v1.177a9.25 9.25 0 0 1-2.517 6.345l-1.228 1.304a22.248 22.248 0 0 1-3.854 3.256l-3.288 2.192-1.743-1.858a.711.711 0 0 0-.034-.034l-1.859-1.744 2.193-3.29a22.25 22.25 0 0 1 3.255-3.85l1.304-1.23ZM17.5 8.657a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm-11 13c.9-.9.9-2.6 0-3.5-.9-.9-2.6-.9-3.5 0-1.209 1.209-1.445 3.901-1.49 4.744a.232.232 0 0 0 .247.246c.842-.045 3.534-.281 4.743-1.49Z"
    />
  </svg>
);

export default Rocket;