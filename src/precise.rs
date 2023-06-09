use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{AddAssign, DivAssign, SubAssign, Add, Mul, Div, Sub};
use buckets::bucketize::BucketizeSingle;
use num_traits::Bounded;
use crate::honest_peer::{HonestPeer, Update};

/// A struct to track local and global trust of peers in a 
/// peer to peer data sharing network. Trust scores 
/// can be incremented or decremented, when the node holding this 
/// struct witnesses trustworthy or malicious behaviours by a peer 
/// respectively. 
///
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::ops::{AddAssign, DivAssign, Add, Mul};
///
/// pub struct PreciseHonestPeer<K, V> 
/// where 
///     K: Eq + Hash + Clone,
///     V: AddAssign 
///     + DivAssign 
///     + Add<Output = V> 
///     + Mul<Output = V> 
///     + Copy 
///     + Default
/// {
///     local_trust: HashMap<K, V>,
///     global_trust: HashMap<K, V>,
///     normalized_local_trust: HashMap<K, V>,
///     normalized_global_trust: HashMap<K, V>,
/// }
/// ```
pub struct PreciseHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign 
    + DivAssign 
    + Add<Output = V> 
    + Mul<Output = V> 
    + Div<Output = V> 
    + Sub<Output = V> 
    + PartialOrd
    + Copy 
    + Default
{
    local_trust: HashMap<K, V>,
    global_trust: HashMap<K, V>,
    normalized_local_trust: HashMap<K, V>,
    normalized_global_trust: HashMap<K, V>,
}


impl<K, V> PreciseHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign 
    + DivAssign 
    + SubAssign 
    + Add<Output = V> 
    + Mul<Output = V> 
    + Div<Output = V> 
    + Sub<Output = V> 
    + PartialOrd
    + Copy 
    + Default,
{
    /// Creates a new `PreciseHonestPeer` struct with no peers in it.
    /// 
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// use decentrust::honest_peer::HonestPeer;
    /// use ordered_float::OrderedFloat;
    ///
    /// let mut hp: PreciseHonestPeer<String, OrderedFloat<f64>> = {
    ///     PreciseHonestPeer::new()
    /// };
    /// 
    /// assert_eq!(0, hp.local_raw_len());
    /// assert_eq!(0, hp.global_raw_len());
    /// ```
    pub fn new() -> Self {
        PreciseHonestPeer { 
            local_trust: HashMap::new(), 
            global_trust: HashMap::new(),
            normalized_local_trust: HashMap::new(),
            normalized_global_trust: HashMap::new(),
        }
    }

    ///
    ///
    pub fn bucketize_local<'a, B>(
        &'a self, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        self.local_trust.iter().map(move |(k, v)| {
            let bucketed = bucketizer.bucketize(v);
            (k.clone(), bucketed)
        })
    }

    ///
    pub fn bucketize_normalized_local<'a, B>(
        &'a self, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        self.normalized_local_trust.iter().map(move |(k, v)| {
            let bucketed = bucketizer.bucketize(v);
            (k.clone(), bucketed)

        })
    }

    pub fn bucketize_global<'a, B>(
        &'a self, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        self.global_trust.iter().map(move |(k, v)| {
            let bucketed = bucketizer.bucketize(v);
            (k.clone(), bucketed)

        })
    }

    pub fn bucketize_normalized_global<'a, B>(
        &'a self, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        self.normalized_global_trust.iter().map(move |(k, v)| {
            let bucketed = bucketizer.bucketize(v);
            (k.clone(), bucketed)
        })
    }
}


impl<K, V> HonestPeer for PreciseHonestPeer<K, V> 
where 
    K: Eq + std::hash::Hash + Clone,  
    V: AddAssign 
        + DivAssign 
        + SubAssign 
        + Add<Output = V> 
        + Mul<Output = V> 
        + Div<Output = V> 
        + Sub<Output = V> 
        + Copy 
        + Default 
        + Bounded 
        + Hash 
        + Ord

{
    type Map = HashMap<K, V>;
    type Key =  K;
    type Value = V; 

    /// Initialize the local trust score of a newly discovered peer 
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// use decentrust::honest_peer::HonestPeer;
    /// use ordered_float::OrderedFloat;
    /// 
    /// let mut hp: PreciseHonestPeer<String, OrderedFloat<f64>> = {
    ///     PreciseHonestPeer::new()
    /// };
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_local(&"node1".to_string(), 0.01f64.into());
    ///
    /// assert_eq!(hp.local_raw_len(), 1);
    /// assert_eq!(hp.local_normalized_len(), 1);
    ///
    /// ```
    fn init_local(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.local_trust.insert(key.clone(), init_value);
        self.normalize_local();
    }

    /// Updates the local trust score of a peer, and normalizes 
    /// the trust score map.
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use ordered_float::OrderedFloat;
    ///
    /// fn equal_floats(a: f64, b: f64, epsilon: f64) -> bool {
    ///     (a - b).abs() < epsilon
    /// }
    ///
    /// let mut hp: PreciseHonestPeer<String, OrderedFloat<f64>> = {
    ///     PreciseHonestPeer::new()
    /// };
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_local(&"node1".to_string(), 0.01f64.into());
    /// hp.init_local(&"node2".to_string(), 0.01f64.into());
    ///
    /// hp.update_local(&"node1".to_string(), 0.05f64.into(), Update::Increment);
    ///
    /// let local_total_trust = 0.01 + 0.01 + 0.05;
    /// let node_1_local_trust: OrderedFloat<f64> = (0.06 / local_total_trust).into();
    /// let node_2_local_trust: OrderedFloat<f64> = (0.01 / local_total_trust).into();
    ///
    /// // Since the init calls are the first in this instance,
    /// // normalization will necessarily mean that they equal 100% of the 
    /// // weight of the scores.
    /// if let Some(val) = hp.get_normalized_local(&"node1".to_string()) {
    ///     assert!(equal_floats(
    ///         val.into_inner(), 
    ///         node_1_local_trust.into_inner(), 
    ///         1e-9f64)
    ///     );
    /// }
    ///
    /// if let Some(val) = hp.get_raw_local(&"node1".to_string()) {
    ///     assert!(equal_floats(val.into_inner(), 0.06f64, 1e-9f64));
    /// }
    ///
    /// if let Some(val) = hp.get_normalized_local(&"node2".to_string()) {
    ///     assert!(equal_floats(
    ///         val.into_inner(), 
    ///         node_2_local_trust.into_inner(),
    ///         1e-9f64)
    ///     );
    /// }
    ///
    /// if let Some(val) = hp.get_raw_local(&"node2".to_string()) {
    ///     assert!(equal_floats(val.into_inner(), 0.01f64, 1e-9f64));
    /// }
    ///
    /// ```
    fn update_local(
        &mut self, 
        key: &Self::Key, 
        trust_delta: Self::Value, 
        update: Update
    ) {
        match update {
            Update::Increment => {
                if let Some(trust_score) = self.local_trust.get_mut(key) {
                    *trust_score += trust_delta
                } else {
                    self.local_trust.insert(key.clone(), trust_delta);
                }
            },
            Update::Decrement => {
                if let Some(trust_score) = self.local_trust.get_mut(key) {
                    if trust_delta > *trust_score {
                        *trust_score = V::default();
                    } else {
                        *trust_score -= trust_delta
                    }
                } else {
                    self.local_trust.insert(key.clone(), trust_delta);
                }
            }
        }
        self.normalize_local()
    }

    /// gets a value from the raw local trust map
    fn get_raw_local(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.local_trust.get(key) {
            return Some(*val)
        } 

        return None 
    }

    /// gets a value from the normalized local trust map
    fn get_normalized_local(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.normalized_local_trust.get(key) {
            return Some(*val)
        }
        return None
    }

    /// Initialize the global trust score of a newly discovered peer 
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// use decentrust::honest_peer::HonestPeer;
    /// use ordered_float::OrderedFloat;
    /// 
    /// let mut hp: PreciseHonestPeer<String, OrderedFloat<f64>> = {
    ///     PreciseHonestPeer::new()
    /// };
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_global(&"node1".to_string(), 0.01f64.into());
    ///
    /// assert_eq!(hp.global_raw_len(), 1);
    /// assert_eq!(hp.global_normalized_len(), 1);
    ///
    /// ```
    fn init_global(&mut self, sender: &Self::Key, key: &Self::Key, init_value: Self::Value) {
        let sender_trust = self.get_normalized_local(sender);
        if let Some(sender_trust) = sender_trust {
            let weighted_init = init_value * sender_trust;
            self.global_trust.insert(key.clone(), weighted_init);
            self.normalize_global()
        }
    }

    /// Updates a global trust value for a given peer and normalizes
    /// the normalized global trust map.
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use ordered_float::OrderedFloat;
    ///
    /// let mut hp: PreciseHonestPeer<String, OrderedFloat<f64>> = PreciseHonestPeer::new();
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_global(&"node1".to_string(), 0.01f64.into());
    /// hp.init_global(&"node2".to_string(), 0.01f64.into());
    ///
    /// hp.update_global(
    ///     &"node2".to_string(), 
    ///     &"node1".to_string(), 
    ///     0.02f64.into(), 
    ///     Update::Increment
    /// );
    ///
    /// let global_total_trust = 0.01 + 0.01 + 0.02;
    /// let node_1_global_trust: OrderedFloat<f64> = (0.03 / global_total_trust).into();
    /// let node_2_global_trust: OrderedFloat<f64> = (0.01 / global_total_trust).into();
    ///
    /// // Since the init calls are the first in this instance,
    /// // normalization will necessarily mean that they equal 100% of the 
    /// // weight of the scores.
    /// println!("{:?}", hp.get_normalized_global(&"node1".to_string())); 
    /// println!("{:?}", hp.get_raw_global(&"node1".to_string())); 
    /// println!("{:?}", hp.get_normalized_global(&"node2".to_string())); 
    /// println!("{:?}", hp.get_raw_global(&"node2".to_string())); 
    /// ```
    fn update_global(
        &mut self, 
        sender: &Self::Key,
        key: &Self::Key, 
        trust_delta: Self::Value, 
        update: Update
    ) {
        let sender_trust = self.get_normalized_local(sender);
        if let Some(sender_trust) = sender_trust {
            let weighted_delta = trust_delta * sender_trust;
            match update {
                Update::Increment => {
                    if let Some(trust_score) = self.global_trust.get_mut(key) {
                        *trust_score += weighted_delta;
                    } else {
                        self.global_trust.insert(key.clone(), weighted_delta);
                    }
                },
                Update::Decrement => {
                    if let Some(trust_score) = self.global_trust.get_mut(key) {
                        if weighted_delta > *trust_score {
                            *trust_score = V::default();
                        } else {
                            *trust_score -= weighted_delta;
                        }
                    }
                }
            }
        }

        self.normalize_global();
    }

    /// gets the raw global trust value for a given peer
    fn get_raw_global(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.global_trust.get(key) {
            return Some(*val)
        }

        return None

    }

    /// gets the normalized global trust value for a given peer
    fn get_normalized_global(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.normalized_global_trust.get(key) {
            return Some(*val)
        }

        return None
    }

    /// returns the entire raw local trust map from the `PreciseHonestPeer` instance
    fn get_raw_local_map(&self) -> Self::Map {
        self.local_trust.clone()
    }

    /// returns the entire normalized local trust map from the `PreciseHonestPeer` instance
    fn get_normalized_local_map(&self) -> Self::Map {
        self.normalized_local_trust.clone()
    }

    /// returns the entire raw global trust map from the `PreciseHonestPeer` instance
    fn get_raw_global_map(&self) -> Self::Map {

        self.global_trust.clone()
    }

    /// returns the entire normalized global trust map from the `PreciseHonestPeer` instance
    fn get_normalized_global_map(&self) -> Self::Map {
        self.normalized_global_trust.clone()
    }

    /// normalizes all the local trust values after a new entry or update 
    /// to an existing entry, and saves them in the `normalized_local_trust` 
    /// map.
    fn normalize_local(&mut self) {
        let total_trust = self.local_trust.values()
            .cloned()
            .fold(V::default(), |acc, x| acc + x);

        self.local_trust.iter().for_each(|(k, v)| {
            let normalized_trust = *v / total_trust;
            self.normalized_local_trust.insert(k.clone(), normalized_trust);
        });
    }

    /// normalizes all the global trust values after a new entry of update 
    /// to an existing entry and saves them in the `normalized_global_trust`
    /// map
    fn normalize_global(&mut self) {
        let total_trust = self.global_trust.values()
            .cloned()
            .fold(V::default(), |acc, x| acc + x);

        self.global_trust.iter_mut().for_each(|(k, v)| {
            let normalized_trust = *v / total_trust;
            self.normalized_global_trust.insert(k.clone(), normalized_trust);
        });
    }

    /// returns the number of key, value pairs in the raw local trust map 
    fn local_raw_len(&self) -> usize {
        self.local_trust.len()
    }

    /// returns the number of key, value pairs in the normalized local trust map 
    fn local_normalized_len(&self) -> usize {
        self.normalized_local_trust.len()
    }

    /// returns the number of key, value pairs in the raw global trust map 
    fn global_raw_len(&self) -> usize {
        self.global_trust.len()
    }

    /// returns the number of key, value pairs in the normalized global trust map 
    fn global_normalized_len(&self) -> usize {
        self.normalized_global_trust.len()
    }
}
