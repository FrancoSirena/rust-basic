[Tree]
# Structures
  ## Tree
  Keeps the tree root and the main methods to add/find and count nodes.
  The Tree itself keeps a lifetime reference because it does have a reference to a Node.

  ## Node
  Each node which keeps the right and left node plus its own value.
  The node has a lifetime reference, meaning we have a single reference for its
  value, so we went run into deallocation issues when working with it.
  We keep it like that so we can safely do our recursions and don't get our pointer pointing to nowhere.
  ### add
    Checks the new value against self and decides it needs to go `left` or `right`.

