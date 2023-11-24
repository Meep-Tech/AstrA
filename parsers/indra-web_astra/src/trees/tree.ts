import { IVertex } from "./vertex";


export class Tree<TNode extends IVertex> {
  root: TNode;
}
