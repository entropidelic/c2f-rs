# Resources

A reading list for understanding fire spread modelling and C2F-W, roughly from intuition
to implementation details.

## 1. Intuition

- **Sullivan (2009), *Wildland surface fire spread modelling, 1990–2007*, part 3: simulation models.**
  The best single overview of how fire spread is simulated: cell grids, Huygens-style
  wave fronts, shortest-path methods. https://arxiv.org/abs/0706.4130
  Parts 1 (physical models) and 2 (empirical models): https://arxiv.org/abs/0706.3074,
  https://arxiv.org/abs/0706.4128
- **NWCG Fire Behavior Field Reference Guide (PMS-437).** Practical explanations of the fire
  triangle (fuel, weather, topography), elliptical fire shapes, intensity and flame length.
  https://gacc.nifc.gov/oncc/docs/FBFRG_2014.pdf

## 2. Cell2Fire and C2F-W

- **Pais et al. (2021), *Cell2Fire: A Cell-Based Forest Fire Growth Model to Support Strategic
  Landscape Management Planning*.** The main paper: grid of cells, an ellipse in every cell,
  messages to neighbours, validation on the Dogrib fire.
  https://www.frontiersin.org/journals/forests-and-global-change/articles/10.3389/ffgc.2021.692706/full
- **C2F-W tutorial slides** (`C2FS&B_Tutorial.pdf` in the C2F-W repository). How to run the
  Scott & Burgan version: inputs, flags, outputs. A usage manual, not theory.
- **fire2a documentation.** https://fire2a.github.io/docs/
- **González-Olabarria et al. (2023), *A fire spread simulator to support tactical management
  decisions for Mediterranean landscapes*.** The Scott & Burgan variant implemented in C2F-W.
  https://doi.org/10.3389/ffgc.2023.1071484
- **Pais et al. (2019), *Adjusting rate of spread factors through derivative-free optimization*.**
  Why cell-based fire shapes get distorted, and the correction factors (`--HFactor` ...).
  https://arxiv.org/abs/1909.05949
- **Kim, Pais, González (2025), *Fire spread simulations using Cell2Fire on synthetic and real
  landscapes*.** Compares Cell2Fire with Prometheus and FARSITE.
  https://pmc.ncbi.nlm.nih.gov/articles/PMC12254405/

Note: C2F-W's Scott & Burgan model does not evaluate the Rothermel equations. It uses curves
fitted per fuel model and moisture scenario. The Rothermel references below explain the
physics behind those curves, not C2F-W's exact numbers.

## 3. Fire behaviour models

- **Anderson (1983), INT-305.** Elliptical fire shape and the length-to-breadth ratio.
  https://www.frames.gov/documents/behaveplus/publications/Anderson_1983_INT-RP-305.pdf
- **Pyretechnics documentation.** Rothermel's surface fire, crown fire and fire spread,
  explained alongside the code. https://pyregence.github.io/pyretechnics/
- **Andrews (2018), RMRS-GTR-371, *The Rothermel surface fire spread model and associated
  developments*.** The definitive guide: every equation, unit and later correction.
  https://www.frames.gov/catalog/26103
- **Rothermel (1972), INT-115.** The original surface fire spread model.
  https://www.frames.gov/documents/behaveplus/publications/Rothermel_1972_INT-RP-115.pdf
- **Scott & Burgan (2005), RMRS-GTR-153.** The 40 standard fuel models (GR1, SH2, TL3 ...)
  and the moisture scenarios. https://www.frames.gov/catalog/792
- **Forestry Canada (1992), ST-X-3.** The Canadian FBP system, used by the original Cell2Fire.
  https://www.frames.gov/documents/catalog/forestry_canada_fire_danger_group_1992.pdf
- **Wotton, Alexander, Taylor (2009), GLC-X-10.** Updates to FBP.
  https://publications.gc.ca/collections/collection_2010/nrcan/Fo123-2-10-2009-eng.pdf
- **Crown fire:** Van Wagner (1977), crown fire initiation and spread,
  https://doi.org/10.1139/x77-004 (paywalled); Rothermel (1991), INT-438,
  https://www.frames.gov/documents/behaveplus/publications/Rothermel_1991_INT-RP-438.pdf;
  Scott & Reinhardt (2001), RMRS-RP-29, https://www.fs.usda.gov/rm/pubs/rmrs_rp029.pdf
- **Kitral (Chile):** Pedernera et al., *Improving the economic efficiency of combatting
  forest fires in Chile: the KITRAL system*, PSW-GTR-173.
  https://www.fs.usda.gov/psw/publications/documents/psw_gtr173/psw_gtr173_03_pedernera.pdf

## 4. Other simulators

- **Finney (1998), RMRS-RP-4, FARSITE.** Vector (Huygens) fire growth, the main alternative
  to cell grids. https://www.frames.gov/documents/behaveplus/publications/Finney_1998_RMRS-RP-4.pdf
- **Finney (2002), minimum travel time.** https://www.frames.gov/catalog/9843
- **ELMFIRE documentation.** A level-set simulator with a technical reference and validation
  on the same Dogrib fire as Cell2Fire. https://elmfire.io/

## 5. Reference implementations and test data

Useful to validate future additions independently from C2F-W:

- **firelab/behave** (C++, the BehavePlus engine). `src/testBehave/testBehave.cpp` has expected
  values for Rothermel, ellipse dimensions, crown fire and intensity.
  https://github.com/firelab/behave
- **pyregence/gridfire**. `test/gridfire/behaveplus_results.clj` tabulates BehavePlus outputs
  for all 40 Scott & Burgan fuel models. https://github.com/pyregence/gridfire
- **cffdrs_py**. Reference CSVs for every FBP function. https://github.com/cffdrs/cffdrs_py
- **cffdrs-rs**. A Rust crate for the Canadian FWI and FBP systems.
  https://github.com/calebissharp/cffdrs-rs
