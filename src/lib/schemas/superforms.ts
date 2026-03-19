import { zodClient, type ClientValidationAdapter } from "sveltekit-superforms/adapters";
import type { z } from "zod";

export function typedZodClient<TSchema extends z.ZodTypeAny>(
	schema: TSchema
): ClientValidationAdapter<Partial<z.infer<TSchema>>, Record<string, unknown>> {
	return zodClient(schema as never) as ClientValidationAdapter<
		Partial<z.infer<TSchema>>,
		Record<string, unknown>
	>;
}
