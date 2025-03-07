//! A [Stochastic Gradient Descent with Momentum][1]
//! [1]: https://en.wikipedia.org/wiki/Stochastic_gradient_descent#Momentum
//!
//! Momentum in solving neural networks works similar to
//! they way it does in physics.
//! If you travel into a a direction with a high velocity,
//! it becomes very hard to change (or reverse)
//! the direction in which you are moving.
//!
//! Similarly when adjusting gradients during solving,
//! keeping a part of the previous gradient update can make solving faster,
//! since if you keep adjusting the gradients
//! into the same direction you will reach the optimum faster.
//! It also makes solving more stable.
use math::*;
use shared_memory::*;
use network::Network;
use solver::*;
use solvers::SGDSolver;

#[derive(Debug, Clone)]
/// Stochastic Gradient Descent with Momentum.
///
/// See [module description][1] for more information.
/// [1]: ./index.html
pub struct Momentum {
    /// The gradient update from the previous iteration for each blob.
    history: Vec<ArcLock<HeapBlob>>,
}

impl Momentum {
    /// Create a new SGD Momentum solver.
    ///
    /// Should not be called directly.
    /// Use [Network::from_config][1] or [Solver::from_config][2] instead.
    ///
    /// [1]: ../../../network/struct.Network.html#method.from_config
    /// [2]: ../../../solver/struct.Solver.html#method.from_config
    pub fn new() -> Momentum {
        Momentum { history: Vec::new() }
    }

    /// Initialize the SGD Momentum solver, allocating memory for its history.
    fn init(&mut self, net: &Network) {
        self.history = Vec::with_capacity(net.learnable_weights().len());

        for weight_blob in net.learnable_weights() {
            let shape = weight_blob.read().unwrap().shape();
            let history_blob = new_shared_heapblob();
            history_blob.write().unwrap().reshape(shape);
            self.history.push(history_blob);
        }
    }
}

impl SGDSolver for Momentum {
    fn compute_update_value(&mut self,
                            config: &SolverConfig,
                            weight_blob: &ArcLock<HeapBlob>,
                            history_blob_id: usize,
                            global_lr: &f32,
                            blob_lr: &f32) {
        let history_blob = &self.history[history_blob_id];
        let momentum = config.momentum;
        let local_lr = global_lr * blob_lr;

        // Compute the update to history, then copy it to the parameter diff.
        leaf_cpu_axpby(&local_lr,
                       weight_blob.read().unwrap().cpu_diff(),
                       &momentum,
                       history_blob.write().unwrap().mutable_cpu_data());
        *weight_blob.write().unwrap().mutable_cpu_diff() = history_blob.read().unwrap().cpu_data().clone();
    }
}

impl_isolver_sgd!(Momentum);
