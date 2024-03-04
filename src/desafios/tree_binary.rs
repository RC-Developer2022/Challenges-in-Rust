#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }
}

fn find_nth_largest(root: &Option<Box<TreeNode>>, n: usize, count: &mut usize) -> Option<i32> {
    if let Some(node) = root {
        // Busca na subárvore da direita primeiro (percurso em ordem decrescente)
        let result = find_nth_largest(&node.right, n, count);

        // Verifica se encontrou o n-ésimo maior elemento
        if result.is_some() {
            return result;
        }

        // Incrementa o contador para o nó atual
        *count += 1;

        // Se o contador atingiu o valor desejado, retorna o valor do n-ésimo maior elemento
        if *count == n {
            return Some(node.value);
        }

        // Busca na subárvore da esquerda
        return find_nth_largest(&node.left, n, count);
    }

    // Retorna None se a árvore estiver vazia
    return None
}