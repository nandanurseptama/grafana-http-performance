import { baseOptions, generateRandomNumber } from "./base.js";

import http from "k6/http";
import { check, sleep } from "k6";

export const options = baseOptions;

export default function () {
  const randomNumber = generateRandomNumber();
  const res = http.get(
    "http://localhost:8080/fibonacci?n=20"
  );
  check(res, { "status was 200": (r) => r.status == 200 });
  sleep(1);
}
