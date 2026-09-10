module Test.Runner where

import Prelude
import Effect (Effect)
import Test.Extra as Extra
import Test.Main as Original

main :: Effect Unit
main = do
  Original.main
  Extra.run
