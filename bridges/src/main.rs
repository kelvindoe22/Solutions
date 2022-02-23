#[derive(PartialEq, Debug)]
enum Light{
    Home,
    Away
}




fn main() {
    let a: Vec<usize> = vec![1,2,5,10,11, 18];
    let mut movements = String::with_capacity(50);
    let mut sum: usize = 0;
    let len_of_a = a.len();
    let mut home = vec![true;len_of_a];
    let mut away = vec![false; len_of_a];
    let mut torch = Light::Home;
    loop {
        let remainder = home
        .iter()
        .enumerate()
        .filter(|(pos,val)| **val == true && (*pos != 0 && *pos != 1))
        .collect::<Vec<_>>();
        match (home[0],home[1]) {
            (true, true) => {
                assert_eq!(torch, Light::Home);
                movements += &*format!("{} {}\n",a[0],a[1]);
                sum += a[1];
                home[0] = false;
                away[0] = true;
                home[1] = false;
                away[1] = true;
                torch = Light::Away;
            }
            (true, false)=> {
                match torch {
                    Light::Home => {
                        match remainder.len() {
                            0 => {
                                movements += &*format!("{}\n",a[0]);
                                sum += a[0];
                                home[0] = false;
                                away[0] = true;
                                break;
                            }
                            1 => {
                                let other = remainder[0].0;
                                movements += &*format!("{} {}\n",a[0],a[other]);
                                sum += a[other];
                                home[0] = false;
                                away[0] = true;
                                home[other] = false;
                                away[other] = true;
                                break;
                            }
                            _ => {
                                let first = remainder[0].0;
                                let second = remainder[1].0;
                                movements += &*format!("{} {}\n",a[first],a[second]);
                                sum += a[second];
                                home[first] = false;
                                away[first] = true;
                                home[second] = false;
                                away[second] = true;
                            }
                        }
                        torch = Light::Away;
                    }
                    Light::Away => {
                        movements += &*format!("{}\n",a[1]);
                        sum += a[1];
                        home[1] = true;
                        away[1] = false;
                        torch = Light::Home;
                    }
                }
            }
            (false, true)=> { // this is not reachable
                match torch {
                    Light::Home => {
                        match remainder.len() {
                            0 => {
                                movements += &*format!("{}\n",a[1]);
                                sum += a[1];
                                home[1] = false;
                                away[1] = true;
                                break;
                            }
                            1 => {
                                let other = remainder[0].0;
                                movements += &*format!("{} {}\n",a[0],a[other]);
                                sum += a[other];
                                home[1] = false;
                                away[1] = true;
                                home[other] = false;
                                away[other] = true;
                                break;
                            }
                            _ => {
                                let first = remainder[0].0;
                                let second = remainder[1].0;
                                movements += &*format!("{} {}\n",a[first],a[second]);
                                sum += a[second];
                                home[first] = false;
                                away[first] = true;
                                home[second] = false;
                                away[second] = true;
                            }
                        }
                        torch = Light::Away;
                    }
                    Light::Away => {
                        movements += &*format!("{}\n",a[0]);
                        sum += a[0];
                        home[0] = true;
                        away[0] = false;
                        torch = Light::Home;
                    }
                }
            }
            (false, false) => {
                assert_eq!(torch, Light::Away);
                if remainder.is_empty() {
                    break;
                }
                movements += &*format!("{}\n",a[0]);
                sum += a[0];
                home[0] = true;
                away[0] = false;
                torch = Light::Home;
            }
        }
    }

    println!("final_results\n{sum}\n{movements}\n");
}


