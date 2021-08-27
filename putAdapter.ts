import type { Address, AgentService, PublicSharing, HolochainLanguageDelegate, LanguageContext } from "@perspect3vism/ad4m";
import { DNA_NICK } from "./dna";

export class EntanglementProofPutAdapter implements PublicSharing {
  #agent: AgentService;
  #holochain: HolochainLanguageDelegate;

  constructor(context: LanguageContext) {
    this.#agent = context.agent;
    this.#holochain = context.Holochain as HolochainLanguageDelegate;
  }

  async createPublic(entanglementProofObject: object): Promise<Address> {
    const orderedEntanglementProofObject = Object.keys(entanglementProofObject)
      .sort()
      .reduce((obj, key) => {
        obj[key] = entanglementProofObject[key];
        return obj;
      }, {});
    const expression = this.#agent.createSignedExpression(orderedEntanglementProofObject);
    const expressionPostData = {
      author: expression.author,
      timestamp: expression.timestamp,
      data: JSON.stringify(expression.data),
      proof: expression.proof,
    };

    //TODO: handle error response and dont just try and parse the object right away
    const res = await this.#holochain.call(
      DNA_NICK,
      "entanglement_proof_store",
      "create_public_expression",
      expressionPostData
    );

    return res.holochain_data.element.signed_header.header.hash.toString("hex");
  }
}
