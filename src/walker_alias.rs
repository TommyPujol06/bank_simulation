use rand::Rng;

pub struct AliasTable {
    len: i64,
    prob: Vec<f64>,
    alias: Vec<usize>,
}

impl AliasTable {
    pub fn new(weights: &[f64]) -> AliasTable {
        let n = weights.len() as i64;

        let sum = weights.iter().fold(0.0, |acc, x| acc + x);

        let mut prob = weights
            .iter()
            .map(|w| w * (n as f64) / sum)
            .collect::<Vec<f64>>();
        let mut h = 0;
        let mut l = n - 1;
        let mut hl: Vec<usize> = vec![0; n as usize];

        for (i, p) in prob.iter().enumerate() {
            if *p < 1.0 {
                hl[l as usize] = i;
                l -= 1;
            }
            if 1.0 < *p {
                hl[h as usize] = i;
                h += 1;
            }
        }

        let mut a: Vec<usize> = vec![0; n as usize];

        while h != 0 && l != n - 1 {
            let j = hl[(l + 1) as usize];
            let k = hl[(h - 1) as usize];

            a[j] = k;
            prob[k] -= 1.0 - prob[j];
            l += 1;
            if prob[k] < 1.0 {
                hl[l as usize] = k;
                l -= 1;
                h -= 1;
            }
        }

        AliasTable {
            len: n,
            prob: prob,
            alias: a,
        }
    }

    pub fn random<R: Rng>(&self, mut rng: R) -> usize {
        let u = rng.gen::<f64>();
        let n = rng.gen_range(0..self.len) as usize;

        if u <= self.prob[n] {
            n
        } else {
            self.alias[n]
        }
    }
}
