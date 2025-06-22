use std::io;

/// 邻接矩阵表示的图结构
/// 
/// # 类型参数
/// - `T`: 顶点数据类型，需要实现Copy、Default和FromStr trait
/// - `N`: 图的顶点数量，编译时常量
/// 
/// # 示例
/// ```
/// use data_structure::linear::graph::AMGraph;
/// 
/// let graph: AMGraph<i32, 3> = AMGraph::from_user_input();
/// println!("顶点: {:?}", graph.vexs_get());
/// ```
#[derive(Debug)]
pub struct AMGraph<T, const N: usize>
where 
    T: Copy + Default + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    vexs: [T; N],
    arcs: [[usize; N]; N],
    arc_num: usize,
}

impl<T, const N: usize> AMGraph<T, N>
where
    T: Copy + Default + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    /// 创建一个新的邻接矩阵图
    /// 
    /// # 参数
    /// - `vexs`: 顶点数组
    /// - `arcs`: 邻接矩阵
    /// - `arc_num`: 边的数量
    pub fn new(vexs: [T; N], arcs: [[usize; N]; N], arc_num: usize) -> Self {
        Self { vexs, arcs, arc_num }
    }

    /// 获取顶点数组
    pub fn vexs_get(&self) -> [T; N] {
        self.vexs
    }

    /// 获取邻接矩阵
    pub fn arcs(&self) -> [[usize; N]; N] {
        self.arcs
    }

    /// 获取边的数量
    pub fn arc_num(&self) -> usize {
        self.arc_num
    }

    /// 从用户输入创建图
    /// 
    /// # 示例
    /// ```
    /// use data_structure::linear::graph::AMGraph;
    /// 
    /// let graph = AMGraph::<i32, 3>::from_user_input();
    /// ```
    pub fn from_user_input() -> Self {
        let mut vexs: [T; N] = [Default::default(); N];
        let mut arcs: [[usize; N]; N] = [[0; N]; N];

        // 读取顶点信息
        println!("请依次输入 {} 个顶点的值:", N);
        for i in 0..N {
            loop {
                println!("请输入第 {} 个顶点的值:", i + 1);
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    println!("读取输入失败: {}", e);
                    continue;
                }

                match input.trim().parse() {
                    Ok(val) => {
                        vexs[i] = val;
                        break;
                    }
                    Err(e) => println!("输入不合法: {:?}，请重新输入", e),
                }
            }
        }

        // 读取边的数量
        let arc_num = loop {
            println!("请输入边的数量 (最大 {}):", N * N);
            let mut input = String::new();
            if let Err(e) = io::stdin().read_line(&mut input) {
                println!("读取输入失败: {}", e);
                continue;
            }

            match input.trim().parse::<usize>() {
                Ok(num) if num <= N * N => break num,
                Ok(_) => println!("边数量不能超过 {}，请重新输入", N * N),
                Err(e) => println!("输入不合法: {}，请重新输入", e),
            }
        };

        // 读取边的信息
        println!("请依次输入每条边的信息 (格式: 起点 终点 权重):");
        for i in 0..arc_num {
            loop {
                println!("第 {} 条边 (剩余 {} 条):", i + 1, arc_num - i);
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    println!("读取输入失败: {}", e);
                    continue;
                }

                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.len() != 3 {
                    println!("需要3个参数 (起点 终点 权重)，请重新输入");
                    continue;
                }

                let (start, end, weight) = match (
                    parts[0].parse::<usize>(),
                    parts[1].parse::<usize>(),
                    parts[2].parse::<usize>(),
                ) {
                    (Ok(s), Ok(e), Ok(w)) if s < N && e < N => (s, e, w),
                    (Ok(_), Ok(_), _) => {
                        println!("权重必须是正整数，请重新输入");
                        continue;
                    }
                    (Ok(_), Err(_), _) => {
                        println!("终点索引必须是0-{}的整数，请重新输入", N - 1);
                        continue;
                    }
                    (Err(_), _, _) => {
                        println!("起点索引必须是0-{}的整数，请重新输入", N - 1);
                        continue;
                    }
                };

                arcs[start][end] = weight;
                break;
            }
        }

        Self { vexs, arcs, arc_num }
    }
}
