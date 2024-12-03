import type { SpyInstance } from "jest-mock";
import { jest, describe, expect, test, beforeAll, beforeEach, afterEach } from "@jest/globals";

const examples = ["jsonstream-status", "jsonstream-blocks"];

describe("JSON stream examples schema tests", () => {
  examples.forEach((dataSource) => {
    describe(`${dataSource}`, () => {
      let consoleLogSpy: SpyInstance;
      let main: () => Promise<void>;
      let schema: any;

      beforeAll(() => {
        main = require(`../examples/${dataSource}.ts`).main;
        schema = require(`./schemas/${dataSource}`).schema;
      });

      beforeEach(() => {
        consoleLogSpy = jest.spyOn(console, "log").mockImplementation(() => {});
      });

      afterEach(() => {
        consoleLogSpy.mockRestore();
      });

      test("should have correct schema", async () => {
        await main();

        consoleLogSpy.mock.calls.forEach(([loggedObj]) => {
          expect(loggedObj).toMatchObject(schema);
        });
      }, 10_000);
    });
  });
});
