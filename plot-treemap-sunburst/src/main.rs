//! Treemap and sunburst: Hierarchical data as a squarified treemap and as a sunburst.

use dataviz::plot::series::frameless::TreeNode;
use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let tree = || {
        vec![
            TreeNode::group(
                "Tech",
                vec![
                    TreeNode::leaf("Apple", 34.0),
                    TreeNode::leaf("Microsoft", 30.0),
                    TreeNode::leaf("Nvidia", 28.0),
                    TreeNode::leaf("Google", 21.0),
                ],
            ),
            TreeNode::group(
                "Finance",
                vec![
                    TreeNode::leaf("JPM", 6.0),
                    TreeNode::leaf("Visa", 5.0),
                    TreeNode::leaf("Bank B", 3.0),
                ],
            ),
            TreeNode::group(
                "Health",
                vec![
                    TreeNode::leaf("Lilly", 7.0),
                    TreeNode::leaf("UNH", 5.0),
                    TreeNode::leaf("J&J", 4.0),
                ],
            ),
            TreeNode::group(
                "Energy",
                vec![TreeNode::leaf("Exxon", 5.0), TreeNode::leaf("Chevron", 3.0)],
            ),
        ]
    };
    let ax = fig.at(0, 0);
    ax.title("Treemap");
    ax.treemap(tree());
    ax.legend(LegendPosition::Hidden);
    let ax = fig.at(0, 1);
    ax.title("Sunburst");
    ax.sunburst(tree());
    ax.legend(LegendPosition::Hidden);

    fig.save("treemap_sunburst.png")?;
    fig.save("treemap_sunburst.svg")?;
    Ok(())
}
