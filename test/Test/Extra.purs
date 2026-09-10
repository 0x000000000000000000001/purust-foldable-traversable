module Test.Extra where

import Prelude

import Data.Either (Either(..))
import Data.Foldable (foldl, foldr)
import Data.FunctorWithIndex (mapWithIndex)
import Data.Identity (Identity(..))
import Data.Maybe (Maybe(..))
import Data.Semigroup.Traversable (sequence1, traverse1, sequence1Default, traverse1Default)
import Data.Traversable (mapAccumL, mapAccumR, scanl, scanr, sequence, traverse)
import Data.TraversableWithIndex (mapAccumLWithIndex, mapAccumRWithIndex, scanlWithIndex, scanrWithIndex, traverseWithIndex)
import Data.Tuple (Tuple(..))
import Effect (Effect, foreachE)
import Effect.Console (log)
import Effect.Exception (throw, try)
import Effect.Ref as Ref
import Test.Assert (assert, assertEqual)
import Test.Main (arrayFrom1UpTo)

checkReplay :: Int -> Effect Unit
checkReplay size = do
  count <- Ref.new 0
  let
    input = arrayFrom1UpTo size
    action = traverse (\n -> do
      previous <- Ref.read count
      assertEqual { actual: n, expected: previous + 1 }
      Ref.write n count
      pure (n * 2)) input
  Ref.read count >>= \actual -> assertEqual { actual, expected: 0 }
  foreachE [1, 2] \_ -> do
    Ref.write 0 count
    result <- action
    assertEqual { actual: result, expected: map (_ * 2) input }
    Ref.read count >>= \actual -> assertEqual { actual, expected: size }

run :: Effect Unit
run = do
  log "[OK] empty folds and traversals"
  assertEqual { actual: foldl (+) 7 ([] :: Array Int), expected: 7 }
  assertEqual { actual: foldr (+) 7 ([] :: Array Int), expected: 7 }
  assertEqual { actual: traverse Just ([] :: Array Int), expected: Just [] }
  assertEqual { actual: sequence ([] :: Array (Array Int)), expected: [[]] }
  assertEqual { actual: mapWithIndex (\i n -> i + n) ([] :: Array Int), expected: [] }

  log "[OK] fold direction and indexed callbacks"
  assertEqual { actual: foldl (-) 10 [1, 2, 3], expected: 4 }
  assertEqual { actual: foldr (-) 10 [1, 2, 3], expected: -8 }
  assertEqual { actual: mapWithIndex (\i n -> i * 10 + n) [3, 4, 5], expected: [3, 14, 25] }
  assertEqual { actual: traverseWithIndex (\i n -> Just (i * 10 + n)) [3, 4, 5], expected: Just [3, 14, 25] }

  log "[OK] balanced traversal: deferred effects, left-to-right order and replay"
  foreachE [0, 1, 2, 3, 4, 5, 10, 100000] checkReplay

  log "[OK] array applicative Cartesian order"
  assertEqual
    { actual: traverse (\n -> [n, n + 10]) [1, 2, 3]
    , expected: [[1, 2, 3], [1, 2, 13], [1, 12, 3], [1, 12, 13], [11, 2, 3], [11, 2, 13], [11, 12, 3], [11, 12, 13]]
    }

  log "[OK] failed traversal stops later effects and can be replayed"
  count <- Ref.new 0
  let action = traverse (\n -> do
        Ref.modify_ (_ + 1) count
        when (n == 2) $ throw "stop traversal"
        pure n) [1, 2, 3, 4]
  foreachE [1, 2] \_ -> do
    Ref.write 0 count
    result <- try action
    assert case result of
      Left _ -> true
      Right _ -> false
    Ref.read count >>= \actual -> assertEqual { actual, expected: 2 }

  log "[OK] traversal accumulation and scan directions"
  let step state n = { accum: state + n, value: state }
  assertEqual { actual: mapAccumL step 0 [1, 2, 3], expected: { accum: 6, value: [0, 1, 3] } }
  assertEqual { actual: mapAccumR step 0 [1, 2, 3], expected: { accum: 6, value: [5, 3, 0] } }
  assertEqual { actual: scanl (-) 10 [1, 2, 3], expected: [9, 7, 4] }
  assertEqual { actual: scanr (-) 10 [1, 2, 3], expected: [-8, 9, -7] }

  log "[OK] indexed accumulation and scans"
  let indexedStep i state n = { accum: state + n + i, value: state }
  assertEqual { actual: mapAccumLWithIndex indexedStep 0 [1, 2, 3], expected: { accum: 9, value: [0, 1, 4] } }
  assertEqual { actual: mapAccumRWithIndex indexedStep 0 [1, 2, 3], expected: { accum: 9, value: [8, 5, 0] } }
  assertEqual { actual: scanlWithIndex (\i state n -> state + n + i) 0 [1, 2, 3], expected: [1, 4, 9] }
  assertEqual { actual: scanrWithIndex (\i n state -> state + n + i) 0 [1, 2, 3], expected: [9, 8, 5] }

  log "[OK] Traversable1 instances and defaults"
  assertEqual { actual: traverse1 Just (Identity 3), expected: Just (Identity 3) }
  assertEqual { actual: sequence1 (Identity (Just 3)), expected: Just (Identity 3) }
  assertEqual { actual: traverse1Default Just (Tuple "kept" 3), expected: Just (Tuple "kept" 3) }
  assertEqual { actual: sequence1Default (Tuple "kept" (Just 3)), expected: Just (Tuple "kept" 3) }
  log "Additional Foldable/Traversable checks passed"
