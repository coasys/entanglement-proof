import type { Address, Expression, ExpressionAdapter, PublicSharing, HolochainLanguageDelegate, LanguageContext } from "@perspect3vism/ad4m";
import { EntanglementProofPutAdapter } from "./putAdapter";
import { DNA_NICK } from "./dna";

export default class Adapter implements ExpressionAdapter {
  #holochain: HolochainLanguageDelegate;

  putAdapter: PublicSharing;

  constructor(context: LanguageContext) {
    this.#holochain = context.Holochain as HolochainLanguageDelegate;
    this.putAdapter = new EntanglementProofPutAdapter(context);
  }

  async get(address: Address): Promise<Expression | null> {
    const hash = Buffer.from(address, "hex");
    const expression = await this.#holochain.call(
      DNA_NICK,
      "entanglement_proof_store",
      "get_expression_by_address",
      hash
    );

    if (expression != null) {
      const ad4mExpression: Expression = Object.assign(
        expression.expression_data
      );
      return ad4mExpression;
    } else {
      return null;
    }
  }
}
