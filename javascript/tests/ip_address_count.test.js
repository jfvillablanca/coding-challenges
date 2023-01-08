const ipsBetween = require("../ip_address_count.js");

test("", () => {
  expect(ipsBetween("150.0.0.0", "150.0.0.1")).toBe(1);
  expect(ipsBetween("10.0.0.0", "10.0.0.50")).toBe(50);
  expect(ipsBetween("20.0.0.10", "20.0.1.0")).toBe(246);
  expect(ipsBetween("10.11.12.13", "10.11.13.0")).toBe(243);
  expect(ipsBetween("160.0.0.0", "160.0.1.0")).toBe(256);
  expect(ipsBetween("170.0.0.0", "170.1.0.0")).toBe(65536);
  expect(ipsBetween("50.0.0.0", "50.1.1.1")).toBe(65793);
  expect(ipsBetween("180.0.0.0", "181.0.0.0")).toBe(16777216);
  expect(ipsBetween("1.2.3.4", "5.6.7.8")).toBe(67372036);
  expect(ipsBetween("0.0.0.0", "255.255.255.255")).toBe(2 ** 32 - 1);
});
